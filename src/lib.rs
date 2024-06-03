mod arrs;
mod block;
mod consts;
mod cursor;
mod gf;
mod img;
pub mod target;

pub mod qr {
    use crate::arrs::{Bit, BitArr, BitArrMethods, Role};
    use crate::block::Block;
    use crate::consts;
    use crate::cursor::Cursor;
    use crate::gf::{self, Field};
    use crate::img::CodeImg;
    use crate::target;
    use image::{ImageBuffer, Rgba};
    use rand::prelude::*;

    const DEBUG: bool = false;
    const DRAW: bool = true;
    const PICTURE: bool = true;
    const NUMBERS_ONLY: bool = false;
    const RANDOM: bool = false;

    // target length is assumed to be less than 256 chars
    // error correction is assumed to be L
    // encoding is assumed to be binary

    fn encode_chars_to_bits(version: u32, url: String) -> BitArr {
        // see https://www.thonky.com/qr-code-tutorial & https://www.nayuki.io/page/creating-a-qr-code-step-by-step
        let required_data_bits = consts::required_data_bits(version);
        let mut data_bits = BitArr::with_capacity(required_data_bits);

        if !NUMBERS_ONLY {
            // byte mode indicators
            data_bits.extend_bits(&consts::BYTE_MODE_IND, Role::Data);

            let char_count_indicator_len = consts::char_count_indicator_len_byte(version);
            let mut char_count_indicator = Vec::with_capacity(char_count_indicator_len);

            let byte = url.len() as u16;

            let mut mask = 1;
            for _ in 0..char_count_indicator_len {
                let bit = ((byte & mask) != 0) as u8;
                char_count_indicator.insert(0, bit);
                mask <<= 1;
            }
            data_bits.extend_bits(&char_count_indicator, Role::Data);

            // encode data_bits
            url.as_bytes().iter().for_each(|b| {
                for j in (0..8).rev() {
                    data_bits.push(Bit {
                        val: (b >> j) & 1 == 1,
                        role: Role::Data,
                    });
                }
            });
        }

        // numeric mode indicators
        data_bits.extend_bits(&consts::NUM_MODE_IND, Role::Data);

        let char_count_indicator_len = consts::char_count_indicator_len_num(version);
        let mut char_count_indicator = Vec::with_capacity(char_count_indicator_len);

        let remaining_space = required_data_bits - (data_bits.len() + 8 + char_count_indicator_len);

        // https://www.thonky.com/qr-code-tutorial/numeric-mode-encoding
        let num_full_groups = remaining_space / 10;
        let remaining_group = match remaining_space % 10 {
            0 => 0,
            n => (n - 1) / 3,
        };

        let byte = (num_full_groups * 3 + remaining_group) as u16;

        if DEBUG {
            println!("numbers added: {byte}");
        }

        let mut mask = 1;
        for _ in 0..char_count_indicator_len {
            let bit = ((byte & mask) != 0) as u8;
            char_count_indicator.insert(0, bit);
            mask <<= 1;
        }
        data_bits.extend_bits(&char_count_indicator, Role::Data);

        // encode placeholder numeric bits
        for _ in 0..num_full_groups {
            data_bits.extend_bits(&[1, 1, 1, 1, 1, 0, 0, 1, 1, 1], Role::Num);
            // 999
        }

        match remaining_group {
            2 => data_bits.extend_bits(&[1, 1, 0, 0, 0, 1, 1], Role::Num), // 99
            1 => data_bits.extend_bits(&[1, 0, 0, 1], Role::Num),          // 9
            _ => (),
        };

        // terminator bits
        data_bits.extend_bits(&[0, 0, 0, 0], Role::Data);

        // bit padding
        if data_bits.len() % 8 != 0 {
            data_bits.extend_bits(&vec![0; 8 - (data_bits.len() % 8)], Role::Data);
        }

        assert!(data_bits.len() == required_data_bits);

        data_bits
    }

    fn gen_blocks<'a>(
        bits: BitArr,
        version: u32,
        field: &'a Field,
        generator_poly: &Vec<u8>,
    ) -> Vec<Block<'a>> {
        let number_of_groups = consts::number_of_groups(version);

        let mut blocks = Vec::with_capacity(consts::total_blocks(version));

        let data_bits_in_group_1 =
            consts::data_bytes_per_block(version, 1) * consts::number_of_blocks(version, 1) * 8;
        let ec_bytes_per_block = consts::ec_bytes_per_block(version);

        for group_index in 0..number_of_groups {
            let data_bits_per_block =
                consts::data_bytes_per_block(version, group_index as u32 + 1) * 8;
            let number_of_blocks = consts::number_of_blocks(version, group_index as u32 + 1);

            for block_index in 0..number_of_blocks {
                let mut block_bits =
                    BitArr::with_capacity(data_bits_per_block + ec_bytes_per_block);
                let start = group_index * data_bits_in_group_1 + block_index * data_bits_per_block;
                let end = start + data_bits_per_block;
                block_bits.extend_from_slice(&bits[start..end]);

                // TODO: these ec bytes get converted to bits and right back to bytes by the block struct, should be passed in separate from the bits
                let ec_bytes = gf::ec_codewords(&field, &block_bits.to_byte_arr(), generator_poly);
                block_bits.extend_bytes(&ec_bytes, Role::EC);

                blocks.push(Block::new(data_bits_per_block / 8, field, block_bits));
            }
        }

        blocks
    }

    pub fn preview(
        version: u32,
        path: String,
        brightness_threshold: u8,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
        let side_len = crate::consts::side_len_of_version(version) as usize;

        let target = target::get_target_scale(path, side_len)?;

        let mut module_info = Vec::with_capacity(side_len * side_len);

        for y in 0..side_len {
            for x in 0..side_len {
                module_info.push((x, y, target[y][x].0, target[y][x].1))
            }
        }

        module_info.sort_by(|a, b| a.2.cmp(&b.2).reverse());

        let mut result = image::ImageBuffer::new(side_len as u32, side_len as u32);

        let black = Rgba([0, 0, 0, 255]);
        let white = Rgba([255, 255, 255, 255]);
        let gray = Rgba([128, 128, 128, 255]);

        let controllable_pixels = consts::numeric_char_capacity(version) * 3;

        for module in module_info[0..controllable_pixels].iter() {
            result.put_pixel(
                module.0 as u32,
                module.1 as u32,
                if module.3 > brightness_threshold {
                    white
                } else {
                    black
                },
            )
        }

        for module in module_info[controllable_pixels..].iter() {
            result.put_pixel(module.0 as u32, module.1 as u32, gray)
        }

        Ok(result)
    }

    // TODO: cache the brightness & contrast produced by the preview function, and use that to build a code
    // pub fn build_from_preview(&self, url: String, module_size: u32) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {}

    // TODO: module size should be a const, it doesnt really matter what the module size is to the user
    pub fn build(
        version: u32,
        mut url: String,
        module_size: u32,
        path: String,
        brightness_threshold: u8,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
        // version + url validation
        if url.chars().any(|x| x >= '\u{00FF}') {
            return Err(String::from("url cannot be encoded as ISO 8859-1!"));
        }
        if version > 40 || version == 0 {
            return Err(String::from("version must be from 1 to 40!"));
        }

        let field = Field::new();
        let generator_poly = gf::gen_poly(&field, consts::ec_bytes_per_block(version));
        url.push('#');

        // data + ec encoding
        println!("encoding data...");
        let data_bits = encode_chars_to_bits(version, url);

        if DEBUG {
            let data_bytes = data_bits.to_byte_arr();
            println!("{:02X?}", data_bytes);
        }

        // create the code image
        let side_length = consts::side_len_of_version(version);
        let black = Rgba([0, 0, 0, 255]);
        let white = Rgba([255, 255, 255, 255]);
        let reserved = Rgba([0, 0, 255, 255]);
        let border = 10 * module_size;
        let mut code = CodeImg::new(
            module_size,
            side_length,
            black,
            white,
            reserved,
            version,
            border,
        );

        let debug_colors = vec![
            Rgba([240, 75, 75, 255]),
            Rgba([240, 240, 75, 255]),
            Rgba([75, 240, 75, 255]),
            Rgba([75, 240, 240, 255]),
            Rgba([75, 75, 240, 255]),
            Rgba([240, 75, 240, 255]),
        ];

        // create the code image navigator
        let mut cursor = Cursor::new(&mut code, side_length);

        // TODO: when block structs are generated, all module roles are lost. if the navigator could tell which modules could not be
        // edited and put those down, instead of trying to have the block set them, it would save some time
        println!("generating blocks...");
        let mut blocks = gen_blocks(data_bits, version, &field, &generator_poly);

        struct Module {
            x: u32,
            y: u32,
            bit_index: usize,
            block_index: usize,
            target_color: bool,
            mask: bool,
            contrast: u32,
        }

        let mut module_info = Vec::with_capacity((side_length * side_length) as usize);

        let target_arr;
        let (color, contrast): (
            Box<dyn Fn(usize, usize) -> bool>,
            Box<dyn Fn(usize, usize) -> u32>,
        ) = if DRAW && PICTURE {
            println!("processing image...");

            // array of contrasts and image length for each pixel corresponding to the target image
            target_arr = target::get_target_scale(path, side_length as usize)?;

            (
                Box::new(|x: usize, y: usize| (&target_arr)[y][x].1 < brightness_threshold),
                Box::new(|x: usize, y: usize| (&target_arr)[y][x].0),
            )
        } else {
            (
                Box::new(|x: usize, y: usize| x % 2 == 1 || y % 2 == 1),
                Box::new(|_x: usize, _y: usize| 0),
            )
        };

        // TODO: this block scope is clunky, this could be done better with good lifetime annotations for the block iterators
        println!("mapping modules...");
        {
            let mut block_data_iters = Vec::new();
            let mut block_ec_iters = Vec::new();

            blocks.iter().for_each(|block| {
                let (data_iter, ec_iter) = block.iter_data_ec();
                block_data_iters.push(data_iter);
                block_ec_iters.push(ec_iter);
            });

            let mut cursor_result = true;

            // TODO: figure out what the proper number for this is
            let num_data_bytes = (consts::required_data_bits(version) / 8)
                + consts::number_of_blocks(version, 2) * 8;

            let num_ec_bytes = consts::ec_bytes_per_block(version) * consts::total_blocks(version);

            // data bytes have to be interleaved completely before the ec bytes can be interleaved
            for data_or_ec in 0..2 {
                let (limit, block_iters) = if data_or_ec == 0 {
                    (num_data_bytes, &mut block_data_iters)
                } else {
                    (num_ec_bytes, &mut block_ec_iters)
                };

                for byte_index in 0..limit {
                    let block_index = byte_index % block_iters.len();

                    // these are to ensure the bytes are interleaved properly
                    let mut display = true;
                    let mut sum = 0;

                    for _ in 0..8 {
                        if let Some((bit_index, bit)) = block_iters[block_index].next() {
                            let mask = cursor.y % 2 == 1;

                            sum <<= 1;
                            sum += bit;

                            module_info.push(Module {
                                x: cursor.x,
                                y: cursor.y,
                                bit_index,
                                block_index,
                                target_color: color(cursor.x as usize, cursor.y as usize),
                                mask,
                                contrast: contrast(cursor.x as usize, cursor.y as usize),
                            });

                            if DEBUG {
                                cursor.place_debug(debug_colors[block_index % debug_colors.len()]);
                            }

                            cursor_result = cursor.next()?
                        } else {
                            display = false
                        }
                    }
                    if DEBUG && display {
                        print!("{:02X} ", sum);
                    }
                }

                assert!(block_iters.iter_mut().all(|iter| iter.next().is_none()));
            }

            while cursor_result {
                cursor.place((cursor.y + 1) % 2 == 0);
                cursor_result = cursor.next()?
            }
        }

        if DEBUG {
            code.save();
        }

        if DRAW {
            let mut rng = rand::thread_rng();
            if RANDOM {
                module_info.shuffle(&mut rng);
            } else {
                module_info.sort_by(|a, b| a.contrast.cmp(&b.contrast).reverse());

                let mut zero_index = module_info.len();

                for i in (0..module_info.len()).rev() {
                    if module_info[i].contrast != 0 {
                        break;
                    } else {
                        zero_index = i;
                    }
                }

                module_info[zero_index..].shuffle(&mut rng);
            }

            // TODO: conversion between boolean and u8 is ugly and doesnt make much sense
            // the type for the color of a module should stay consistent across the entire program
            println!("setting module colors...");
            module_info.iter().for_each(|module| {
                blocks[module.block_index]
                    .set(module.bit_index, (module.target_color == module.mask) as u8);
            });

            let mut choice = 0;

            // do while loop
            println!("checking for errors...");
            while {
                let mut errors = Vec::new();

                if DEBUG {
                    println!("start of number correction loop")
                };

                // TODO: another block scope because of the shitty lifetime annotations for the iterators
                {
                    // produces an iterator for the data bits representing numbers, in the form of (block_index, bit_index, bit)
                    let mut numeric_data_iter = blocks
                        .iter()
                        .enumerate()
                        .map(|(block_index, block)| {
                            block
                                .iter_nums()
                                .map(move |(bit_index, bit)| (block_index, bit_index, bit))
                        })
                        .flatten()
                        .peekable();

                    while numeric_data_iter.peek() != None {
                        // parse the data
                        let (block_index, bit_index, bit) = numeric_data_iter.next().unwrap();

                        let mut indexes = Vec::with_capacity(10);
                        indexes.push((block_index, bit_index));

                        let mut val = bit as u16;
                        let mut reached = 1;
                        for _ in 0..9 {
                            if let Some((block_index, bit_index, bit)) = numeric_data_iter.next() {
                                val <<= 1;
                                val += bit as u16;
                                reached += 1;
                                indexes.push((block_index, bit_index));
                            } else {
                                break;
                            }
                        }
                        // deal with errors
                        let compareval = match reached {
                            10 => 999,
                            7 => 99,
                            4 => 9,
                            _ => panic!("wrong number of numeric bits"),
                        };

                        if DEBUG {
                            match reached {
                                10 => print!("{:03}", val),
                                7 => print!("{:02}", val),
                                4 => print!("{:01}", val),
                                _ => (),
                            };
                        }

                        if val > compareval {
                            // TODO: make this choice between the bits to flip random
                            choice = (choice + 1) % 5;

                            errors.push(indexes[choice % indexes.len()]);
                        }
                    }
                }
                errors
                    .iter()
                    .for_each(|(block_index, bit_index)| blocks[*block_index].reset(*bit_index));

                if DEBUG {
                    println!("{:?}", errors);
                }

                errors.len() != 0
            } {}
        }

        if DEBUG {
            blocks.iter().for_each(|b| b.debug());
        }

        let module_values = blocks
            .into_iter()
            .map(|b| return b.ret())
            .collect::<Vec<_>>();

        module_info.iter().for_each(|module| {
            code.fill_module(
                module.x,
                module.y,
                (module_values[module.block_index][module.bit_index] == 1) == module.mask,
            );
        });

        Ok(code.image())
    }
}
