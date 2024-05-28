mod consts;
mod cursor;
mod gf;
mod img;

pub mod qr {
    use crate::consts::{self, required_data_bits};
    use crate::cursor::Cursor;
    use crate::gf::{self, Field};
    use crate::img::CodeImg;
    use image::{ImageBuffer, Pixel, Rgba};

    const DEBUG: bool = false;
    // extend url with 0xff instead of padding bytes
    // phone cannot scan past v12
    const PAD: bool = false;

    // fn bytes_to_bits(bytes: &[u8]) -> Vec<u8> {
    //     bytes
    //         .iter()
    //         .fold(Vec::with_capacity(bytes.len() * 8), |mut v, byte| {
    //             let mut mask = 0b10000000;
    //             for _ in 0..8 {
    //                 let bit = ((byte & mask) != 0) as u8;
    //                 v.push(bit);
    //                 mask >>= 1;
    //             }
    //             v
    //         })
    // }
    fn bits_to_bytes(bytes: &[u8]) -> Vec<u8> {
        (0..(bytes.len() / 8)).fold(Vec::with_capacity(bytes.len() / 8), |mut v, i| {
            let i = i * 8;
            let mut s: u8 = 0;
            let mut mask = 0b10000000;
            for j in 0..8 {
                s += bytes[i + j] * mask;
                mask >>= 1;
            }
            v.push(s);
            v
        })
    }

    pub struct Code {
        url: String,
        version: u32,
        field: Field,
    }
    // target length is assumed to be less than 256 chars
    // error correction is assumed to be L
    // encoding is assumed to be binary (!TODO)

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

        fn encode_chars_to_bits(&self) -> Vec<u8> {
            // see https://www.thonky.com/qr-code-tutorial
            let required_data_bits = consts::required_data_bits(self.version);
            let mut data_bits: Vec<u8> = Vec::with_capacity(required_data_bits);
            let char_capacity = consts::char_capacity(self.version);

            // indicators
            data_bits.extend_from_slice(&consts::MODE_IND);

            let char_count_indicator_len = consts::char_count_indicator_len(self.version);
            let mut char_count_indicator = Vec::with_capacity(char_count_indicator_len);
            let byte = if PAD {
                self.url.len() as u32
            } else {
                char_capacity as u32
            };
            let mut mask = 1;
            for _ in 0..char_count_indicator_len {
                let bit = ((byte & mask) != 0) as u8;
                char_count_indicator.insert(0, bit);
                mask <<= 1;
            }
            data_bits.extend_from_slice(&char_count_indicator);

            // encode data_bits
            self.url.as_bytes().iter().for_each(|b| {
                for j in (0..8).rev() {
                    data_bits.push((b >> j) & 1);
                }
            });

            if !PAD {
                // after this byte, even if you place a byte that corresponds to the unicode for a
                // character valid in a url, it wont show in the decoded url.
                data_bits.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
                for _ in (self.url.len() - 2)..char_capacity {
                    data_bits.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
                }
            }

            // terminator bits
            data_bits.extend_from_slice(&[0, 0, 0, 0]);

            assert!(data_bits.len() % 8 == 0);

            if PAD {
                let mut i = 0;
                while data_bits.len() < required_data_bits {
                    let byte = consts::pad_bytes(i);
                    data_bits.extend_from_slice(&byte);
                    i += 1;
                }
            }
            data_bits
        }

        fn gen_data<'a>(&self, data_bytes: &'a mut Vec<u8>) -> Vec<&'a mut [u8]> {
            let number_of_groups = consts::number_of_groups(self.version);

            let mut data_blocks = Vec::with_capacity(consts::total_blocks(self.version));

            let mut rest = &mut data_bytes[..];

            for group in 0..number_of_groups {
                let data_bytes_per_block =
                    consts::data_bytes_per_block(self.version, group as u32 + 1);
                let number_of_blocks = consts::number_of_blocks(self.version, group as u32 + 1);

                for _block in 0..number_of_blocks {
                    let (data_block, slice) = rest.split_at_mut(data_bytes_per_block);
                    rest = slice;

                    data_blocks.push(data_block);
                }
            }

            data_blocks
        }

        fn gen_ec(&self, data_blocks: &Vec<&mut [u8]>) -> Vec<Vec<u8>> {
            let ec_bytes_per_block = consts::ec_bytes_per_block(self.version);
            let poly = gf::gen_poly(&self.field, ec_bytes_per_block);

            let mut ec_blocks = Vec::with_capacity(data_blocks.len());

            for block in data_blocks {
                ec_blocks.push(gf::ec_codewords(&self.field, *block, &poly));
            }

            ec_blocks
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
            let mut data_bytes = bits_to_bytes(&data_bits);

            if DEBUG {
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

            let mut debug_colors = vec![
                Rgba([240, 75, 75, 255]),
                Rgba([240, 240, 75, 255]),
                Rgba([75, 240, 75, 255]),
                Rgba([75, 240, 240, 255]),
                Rgba([75, 75, 240, 255]),
                Rgba([240, 75, 240, 255]),
            ];

            // create the code image navigator
            let mut cursor = Cursor::new(&mut code, side_length);

            let mut data_blocks = self.gen_data(&mut data_bytes);
            let mut data_block_iters: Vec<_> = data_blocks
                .iter_mut()
                .map(|slice_ref| slice_ref.iter_mut())
                .collect();

            for byte_i in 0..required_data_bits(self.version) {
                let mut mask = 0b10000000;
                let iter_i = byte_i % data_block_iters.len();
                if let Some(byte) = data_block_iters[iter_i].next() {
                    if DEBUG {
                        print!("{:02X} ", byte);
                    }
                    for _ in 0..8 {
                        let bit = ((*byte & mask) != 0) == ((cursor.y + 1) % 2 == 0);
                        mask >>= 1;

                        if DEBUG {
                            cursor.place_debug(debug_colors[iter_i % 6]);
                        } else {
                            cursor.place(bit);
                        }

                        if !cursor.next()? {
                            return Err(format!(
                                "Premature ending in module placement, at ({}, {})",
                                cursor.x, cursor.y
                            ));
                        }
                    }
                }
            }

            debug_colors.iter_mut().for_each(|color_ref| {
                color_ref.apply_without_alpha(|pix| if pix != 0 { pix - 64 } else { pix })
            });

            let mut ec_blocks = self.gen_ec(&data_blocks);
            let mut ec_block_iters: Vec<_> = ec_blocks
                .iter_mut()
                .map(|slice_ref| (*slice_ref).iter_mut())
                .collect();

            let mut cursor_result = true;

            let ec_bytes_len = ec_block_iters.len() * consts::ec_bytes_per_block(self.version);
            for byte_i in 0..ec_bytes_len {
                let mut mask = 0b10000000;
                let iter_i = byte_i % ec_block_iters.len();
                if let Some(byte) = ec_block_iters[iter_i].next() {
                    if DEBUG {
                        print!("{:02X} ", byte);
                    }
                    for _ in 0..8 {
                        let bit = ((*byte & mask) != 0) == ((cursor.y + 1) % 2 == 0);
                        mask >>= 1;

                        if DEBUG {
                            cursor.place_debug(debug_colors[iter_i % 6]);
                        } else {
                            cursor.place(bit);
                        }

                        cursor_result = cursor.next()?
                    }
                };
            }

            while cursor_result {
                cursor.place((cursor.y + 1) % 2 == 0);
                cursor_result = cursor.next()?
            }

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
