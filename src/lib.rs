mod consts;
mod gf;
mod img;

pub mod qr {
    use crate::consts;
    use crate::gf::{self, Field};
    use crate::img::CodeImg;
    use image::{ImageBuffer, Rgba};

    fn bytes_to_bits(bytes: &[u8]) -> Vec<u8> {
        bytes
            .iter()
            .fold(Vec::with_capacity(bytes.len() * 8), |mut v, byte| {
                let mut mask = 0b10000000;
                for _ in 0..8 {
                    let bit = ((byte & mask) != 0) as u8;
                    v.push(bit);
                    mask >>= 1;
                }
                v
            })
    }
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
        version: u8,
        field: Field,
    }
    // target length is assumed to be less than 256 chars
    // error correction is assumed to be L
    // encoding is assumed to be binary (!TODO)
    // version number is decided by build (version 6)(!TODO)

    impl Code {
        // constructor, takes the information to be encoded in the code
        pub fn new(url: String, version: u8) -> Self {
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

            // indicators
            data_bits.extend_from_slice(&consts::MODE_IND);

            let mut char_count_indicator = Vec::with_capacity(consts::CHAR_CT_IND_LEN);
            let byte = self.url.len() as u8;
            let mut mask = 1;
            for _ in 0..consts::CHAR_CT_IND_LEN {
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

            // terminator bits
            data_bits.extend_from_slice(&[0, 0, 0, 0]);

            assert!(data_bits.len() % 8 == 0);

            //padding
            let mut i = 0;
            while data_bits.len() < required_data_bits {
                let byte = consts::pad_bytes(i);
                data_bits.extend_from_slice(&byte);
                i += 1;
            }
            data_bits
        }

        fn split_and_interleave_bytes(&self, data_bytes: Vec<u8>) -> Vec<u8> {
            // structure final message
            // qr codes v1-v7 at EC L have only 1 group

            let data_bytes_per_block = consts::data_bytes_per_block(self.version);
            let ec_bytes_per_block = consts::ec_bytes_per_block(self.version);
            let number_of_blocks = consts::number_of_blocks(self.version);

            let mut code_bytes: Vec<u8> = Vec::with_capacity(
                (data_bytes_per_block + ec_bytes_per_block) * number_of_blocks + 1,
            );

            // data + interleaving
            let mut data_blocks = Vec::with_capacity(number_of_blocks);

            for block in 0..number_of_blocks {
                let start = block * data_bytes_per_block;
                let end = start + data_bytes_per_block;
                let data_block = data_bytes[start..end].iter();

                data_blocks.push(data_block);
            }

            for _ in 0..data_bytes_per_block {
                for block in data_blocks.iter_mut() {
                    if let Some(byte) = block.next() {
                        code_bytes.push(*byte);
                    }
                }
            }

            // error correction + interleaving
            let poly = gf::gen_poly(&self.field, ec_bytes_per_block);

            let mut ec_blocks = Vec::with_capacity(number_of_blocks);

            for block in 0..number_of_blocks {
                let start = block * data_bytes_per_block;
                let end = start + data_bytes_per_block;
                let ec_block = gf::ec_codewords(&self.field, &data_bytes[start..end], &poly);

                ec_blocks.push(ec_block);
            }

            for i in 0..ec_bytes_per_block {
                for block in ec_blocks.iter() {
                    if let Some(byte) = block.get(i) {
                        code_bytes.push(*byte);
                    }
                }
            }

            // add remainder bits
            // code_bytes.push(0);

            code_bytes
        }

        pub fn build(&self, module_size: u32) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
            // version + url validation
            if self.url.chars().any(|x| x >= '\u{00FF}') {
                return Err(String::from("url cannot be encoded as ISO 8859-1!"));
            }
            if self.version > 7 || self.version == 0 {
                return Err(String::from("version must be from 1 to 7!"));
            }

            // data + ec encoding
            let data_bits = self.encode_chars_to_bits();

            let data_bytes = bits_to_bytes(&data_bits);

            let code_bytes = self.split_and_interleave_bytes(data_bytes);

            // create the code image
            let side_length = consts::side_len_of_version(self.version);
            let black = Rgba([0, 0, 0, 255]);
            let white = Rgba([255, 255, 255, 255]);
            let reserved = Rgba([0, 0, 255, 255]);
            let mut code = CodeImg::new(
                module_size,
                side_length,
                black,
                white,
                reserved,
                self.version,
            );

            // place the data + ec inside the code image
            let mut x = side_length - 1;
            let mut y = side_length - 1;

            let code_bits = bytes_to_bits(&code_bytes);

            enum Move {
                Left,
                UpRight,
                DownRight,
            }

            let mut prev_move = Move::UpRight;
            let mut next_move = Move::Left;

            let debug = false;
            let mut count = 0;
            let mut save = 0;
            for bit in code_bits {
                count += 1;

                let color = (bit == 1) == ((y + 1) % 2 == 0);
                if !debug {
                    code.fill_module(x, y, color);
                } else {
                    if ((count - 1) / 8) % 3 == 0 {
                        code.debug(x, y, Rgba([255, 0, 0, 255]))
                    } else if ((count - 1) / 8) % 3 == 1 {
                        code.debug(x, y, Rgba([0, 255, 0, 255]))
                    } else {
                        code.debug(x, y, Rgba([0, 0, 255, 255]))
                    }
                }

                if save > 0 {
                    if save == 1 {
                        code.save();
                        save = 0;
                    } else {
                        save -= 1;
                    }
                }

                match next_move {
                    Move::Left => {
                        if x != 0 && !code.is_open(x - 1, y) {
                            return Err(format!("No valid moves! at ({},{})", x, y));
                        }
                        x -= 1;
                        match prev_move {
                            Move::Left => {
                                if y != 0 && code.is_open(x + 1, y - 1) {
                                    next_move = Move::UpRight;
                                } else {
                                    next_move = Move::DownRight;
                                }
                                prev_move = Move::Left;
                            }
                            other_move => {
                                next_move = other_move;
                                prev_move = Move::Left;
                            }
                        }
                    }
                    Move::UpRight => {
                        if y != 0 && code.is_open(x + 1, y - 1) {
                            x += 1;
                            y -= 1;
                            next_move = Move::Left;
                            prev_move = Move::UpRight;
                        } else if y >= 1 && code.is_open(x, y - 1) {
                            y -= 1;
                            next_move = Move::UpRight;
                            prev_move = Move::UpRight;
                        } else if y >= 2 && code.is_open(x + 1, y - 2) {
                            x += 1;
                            y -= 2;
                            next_move = Move::Left;
                            prev_move = Move::UpRight;
                        } else if y >= 2 && code.is_open(x, y - 2) {
                            y -= 2;
                            next_move = Move::UpRight;
                            prev_move = Move::UpRight;
                        } else if y >= 6 && code.is_open(x + 1, y - 6) {
                            x += 1;
                            y -= 6;
                            next_move = Move::Left;
                            prev_move = Move::UpRight;
                        } else if y >= 7 && x >= 2 && code.is_open(x - 2, y - 7) {
                            x -= 2;
                            y -= 7;
                            next_move = Move::DownRight;
                            prev_move = Move::DownRight;
                        } else if x >= 1 && code.is_open(x - 1, y) {
                            x -= 1;
                            next_move = Move::Left;
                            prev_move = Move::Left;
                        } else if x >= 2 && code.is_open(x - 2, y) {
                            x -= 2;
                            next_move = Move::Left;
                            prev_move = Move::Left;
                        } else {
                            return Err(format!("No valid moves! at ({},{})", x, y));
                        }
                    }
                    Move::DownRight => {
                        if code.is_open(x + 1, y + 1) {
                            x += 1;
                            y += 1;
                            next_move = Move::Left;
                            prev_move = Move::DownRight;
                        } else if code.is_open(x + 1, y + 2) {
                            x += 1;
                            y += 2;
                            next_move = Move::Left;
                            prev_move = Move::DownRight;
                        } else if code.is_open(x, y + 1) {
                            y += 1;
                            prev_move = Move::DownRight;
                            next_move = Move::DownRight;
                        } else if code.is_open(x + 1, y + 6) {
                            x += 1;
                            y += 6;
                            next_move = Move::Left;
                            prev_move = Move::DownRight;
                        } else if x >= 1 && code.is_open(x - 1, y) {
                            x -= 1;
                            next_move = Move::Left;
                            prev_move = Move::Left;
                        } else if x >= 1 && y >= 8 && code.is_open(x - 1, y - 8) {
                            x -= 1;
                            y -= 8;
                            next_move = Move::Left;
                            prev_move = Move::Left;
                        } else {
                            break;
                        }
                    }
                };
                // if count % 8 == 0 {
                //     std::thread::sleep(std::time::Duration::from_millis(1000));
                //     code.save();
                //     println!("count: {}", count);
                // }
            }

            // place format information
            let format_string = consts::format_string(self.version);

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

            // place version information if applicable
            if self.version >= 7 {
                let version_string = consts::versions_string(self.version);

                for i in 0..6 {
                    for j in 0..3 {
                        code.fill_module(
                            5 - i,
                            ((side_length - 1) - 8) - j,
                            version_string[(i * 3 + j) as usize] != 0,
                        );
                        code.fill_module(
                            ((side_length - 1) - 8) - j,
                            5 - i,
                            version_string[(i * 3 + j) as usize] != 0,
                        );
                    }
                }
            }

            println!("isopen: {}", code.is_open(1, 1));

            Ok(code.image())
        }
    }
}
