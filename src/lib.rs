mod arrs;
mod block;
mod consts;
mod cursor;
mod gf;
mod img;

pub mod qr {
    use crate::arrs::{BitArr, BitArrMethods, Module, Role};
    use crate::block::Block;
    use crate::consts;
    use crate::cursor::Cursor;
    use crate::gf::{self, Field};
    use crate::img::CodeImg;
    use image::{ImageBuffer, Rgba};

    const DEBUG: bool = false;
    const DRAW: bool = !false;

    // target length is assumed to be less than 256 chars
    // error correction is assumed to be L
    // encoding is assumed to be binary
    pub struct Code {
        url: String,
        version: u32,
        field: Field,
    }

    impl Code {
        // constructor, takes the information to be encoded in the code
        pub fn new(url: String, version: u32) -> Self {
            let field = Field::new();
            Self {
                url,
                version,
                field,
            }
        }

        fn encode_chars_to_bits(&self) -> BitArr {
            // see https://www.thonky.com/qr-code-tutorial & https://www.nayuki.io/page/creating-a-qr-code-step-by-step
            let required_data_bits = consts::required_data_bits(self.version);
            let mut data_bits = BitArr::with_capacity(required_data_bits);

            // byte mode indicators
            data_bits.extend_bits(&consts::BYTE_MODE_IND, Role::Data);

            let char_count_indicator_len = consts::char_count_indicator_len_byte(self.version);
            let mut char_count_indicator = Vec::with_capacity(char_count_indicator_len);

            let byte = self.url.len() as u16 + 1;

            let mut mask = 1;
            for _ in 0..char_count_indicator_len {
                let bit = ((byte & mask) != 0) as u8;
                char_count_indicator.insert(0, bit);
                mask <<= 1;
            }
            data_bits.extend_bits(&char_count_indicator, Role::Data);

            // encode data_bits
            self.url.as_bytes().iter().for_each(|b| {
                for j in (0..8).rev() {
                    data_bits.push(Module {
                        val: (b >> j) & 1 == 1,
                        role: Role::Data,
                    });
                }
            });

            // #
            data_bits.extend_bits(&[0, 0, 1, 0, 0, 0, 1, 1], Role::Data);

            // numeric mode indicators
            data_bits.extend_bits(&consts::NUM_MODE_IND, Role::Data);

            let char_count_indicator_len = consts::char_count_indicator_len_num(self.version);
            let mut char_count_indicator = Vec::with_capacity(char_count_indicator_len);

            let remaining_space =
                required_data_bits - (data_bits.len() + 8 + char_count_indicator_len);

            let num_full_groups = remaining_space / 10;
            let remaining_group = if (remaining_space % 10) >= 7 {
                2
            } else if (remaining_space % 10) >= 4 {
                1
            } else {
                0
            };

            let byte = (num_full_groups * 3 + remaining_group) as u16;

            println!("{byte}");

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

            println!("{:02X?}", data_bits.to_byte_arr());

            data_bits
        }

        fn gen_blocks(&self, bits: BitArr) -> Vec<Block> {
            let number_of_groups = consts::number_of_groups(self.version);

            let mut blocks = Vec::with_capacity(consts::total_blocks(self.version));

            let data_bits_in_group_1 = consts::data_bytes_per_block(self.version, 1)
                * consts::number_of_blocks(self.version, 1)
                * 8;
            let ec_bytes_per_block = consts::ec_bytes_per_block(self.version);
            let poly = gf::gen_poly(&self.field, ec_bytes_per_block);

            for group_index in 0..number_of_groups {
                let data_bits_per_block =
                    consts::data_bytes_per_block(self.version, group_index as u32 + 1) * 8;
                let number_of_blocks =
                    consts::number_of_blocks(self.version, group_index as u32 + 1);

                for block_index in 0..number_of_blocks {
                    let mut block_bits =
                        BitArr::with_capacity(data_bits_per_block + ec_bytes_per_block);
                    let start =
                        group_index * data_bits_in_group_1 + block_index * data_bits_per_block;
                    let end = start + data_bits_per_block;
                    block_bits.extend_from_slice(&bits[start..end]);
                    let ec_bytes = gf::ec_codewords(&self.field, &block_bits.to_byte_arr(), &poly);

                    block_bits.extend_bytes(&ec_bytes, Role::EC);
                    blocks.push(Block::new(data_bits_per_block / 8, &self.field, block_bits));
                }
            }

            blocks
        }

        pub fn build(&self, module_size: u32) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
            // version + url validation
            if self.url.chars().any(|x| x >= '\u{00FF}') {
                return Err(String::from("url cannot be encoded as ISO 8859-1!"));
            }
            if self.version > 40 || self.version == 0 {
                return Err(String::from("version must be from 1 to 40!"));
            }

            // data + ec encoding
            let data_bits = self.encode_chars_to_bits();

            if DEBUG {
                let data_bytes = data_bits.to_byte_arr();
                println!("{:02X?}  ////", data_bytes);
            }

            // create the code image
            let side_length = consts::side_len_of_version(self.version);
            let black = Rgba([0, 0, 0, 255]);
            let white = Rgba([255, 255, 255, 255]);
            let reserved = Rgba([0, 0, 255, 255]);
            let border = 20;
            let mut code = CodeImg::new(
                module_size,
                side_length,
                black,
                white,
                reserved,
                self.version,
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

            let mut blocks = self.gen_blocks(data_bits);

            let mut block_iters = blocks
                .iter()
                .map(|block| block.iter().enumerate())
                .collect::<Vec<_>>();

            let mut cursor_result = true;

            struct EditModules {
                mx: u32,
                my: u32,
                bit_index: usize,
                block_index: usize,
                target_color: bool,
                mask: bool,
            }

            let mut editable_modules = Vec::with_capacity((side_length * side_length) as usize);

            'byte: for byte_i in 0.. {
                let iter_i = byte_i % block_iters.len();
                for _ in 0..8 {
                    if let Some((bit_index, bit)) = block_iters[iter_i].next() {
                        let mask = (cursor.y + 1) % 2 == 0;
                        let color = (bit == 1) == mask;

                        // print!("{bit_index} ");

                        editable_modules.push(EditModules {
                            mx: cursor.x,
                            my: cursor.y,
                            bit_index,
                            block_index: iter_i,
                            target_color: true,
                            mask,
                        });

                        cursor_result = cursor.next()?
                    } else {
                        if block_iters.iter().all(|iter| iter.len() == 0) {
                            break 'byte;
                        }
                    }
                }
            }

            while cursor_result {
                cursor.place((cursor.y + 1) % 2 == 0);
                cursor_result = cursor.next()?
            }

            if DRAW {
                editable_modules.iter().for_each(|module| {
                    blocks[module.block_index].set(module.bit_index, module.target_color as u8);
                });

                // do while loop
                while {
                    let mut errors = Vec::new();

                    println!("start of loop");

                    // this is so fucked
                    // (block_index, bit_index, bit)
                    let mut numeric_data_iter = blocks
                        .iter()
                        .enumerate()
                        .map(|(block_index, block)| {
                            block
                                .iter_nums()
                                .enumerate()
                                .map(move |(bit_index, bit)| (block_index, bit_index, bit))
                        })
                        .flatten()
                        .peekable();
                    let mut choice = 0;

                    while numeric_data_iter.peek() != None {
                        // parse the data
                        let (block_index, starting_bit_index, bit) =
                            numeric_data_iter.next().unwrap();
                        let mut val = bit as u16;
                        let mut reached = 1;
                        for _ in 0..9 {
                            if let Some((_, _, bit)) = numeric_data_iter.next() {
                                val <<= 1;
                                val += bit as u16;
                                reached += 1;
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
                        // println!("{val}");

                        if val > compareval {
                            // choice between the 5 most significant bits TODO: random
                            choice = (choice + 1) % 5;

                            errors.push((block_index, starting_bit_index + choice));
                        }
                    }

                    errors.iter().for_each(|(block_index, bit_index)| {
                        blocks[*block_index].reset(*bit_index)
                    });

                    errors.len() != 0
                } {}
            }

            blocks.iter().for_each(|b| b.debug());

            let painted = blocks
                .into_iter()
                .map(|b| {
                    return b.ret();
                })
                .collect::<Vec<_>>();

            editable_modules.iter().for_each(|module| {
                code.fill_module(
                    module.mx,
                    module.my,
                    (painted[module.block_index][module.bit_index] == 1) == module.mask,
                );
            });

            // place format information
            let format_string = consts::FORMAT_STRING;

            for (i, bit) in format_string[..7].iter().enumerate() {
                let color = *bit == 1;
                code.fill_module(8, (side_length - 1) - i as u32, color);
                if code.is_reserved(i as u32, 8) {
                    code.fill_module(i as u32, 8, color);
                } else {
                    code.fill_module(i as u32 + 1, 8, color);
                }
            }
            for (i, bit) in format_string[7..].iter().enumerate() {
                let color = *bit == 1;

                code.fill_module((side_length - 8) + i as u32, 8, color);
                if code.is_reserved(8, 8 - (i as u32)) {
                    code.fill_module(8, 8 - (i as u32), color);
                } else {
                    code.fill_module(8, 8 - (i as u32 + 1), color);
                }
            }

            Ok(code.image())
        }
    }
}
