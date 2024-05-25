mod consts;
mod gf;
mod img;

pub mod qr {
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
    }
    // target length is assumed to be less than 256 chars
    // error correction is assumed to be L
    // encoding is assumed to be binary (!TODO)
    // version number is decided by build (version 6)(!TODO)

    impl Code {
        // constructor, takes the information to be encoded in the code
        pub fn new(url: String) -> Self {
            Self { url }
        }

        fn encode_chars_to_bits(&self) -> Vec<u8> {
            // see https://www.thonky.com/qr-code-tutorial
            let required_data_bits = 8 * 136;
            let mut data_bits: Vec<u8> = Vec::with_capacity(required_data_bits);

            // indicators
            data_bits.extend_from_slice(&[0, 1, 0, 0]);

            // byte mode char count indicator for v6 must be 8 bits
            let mut char_count_indicator = Vec::with_capacity(8);
            let byte = self.url.len() as u8;
            let mut mask = 1;
            for _ in 0..8 {
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
            let mut toggle = true;
            while data_bits.len() < required_data_bits {
                let byte = if toggle {
                    [1, 1, 1, 0, 1, 1, 0, 0]
                } else {
                    [0, 0, 0, 1, 0, 0, 0, 1]
                };

                toggle = !toggle;

                data_bits.extend_from_slice(&byte);
            }
            data_bits
        }

        fn split_and_interleave_bytes(data_bytes: Vec<u8>, f: &Field, poly: &[u8]) -> Vec<u8> {
            // structure final message
            // 6-L:
            // total number of data_bytes codewords: 136
            // EC codewords per block: 18
            // Number of Blocks in Group 1: 2
            // Number of data_bytes codewords in each of Group 1's Blocks: 68

            let mut code_bytes: Vec<u8> = Vec::with_capacity(data_bytes.len() + 18 * 2 + 1);

            // group 1 data + interleaving
            let mut data_blocks = [Vec::with_capacity(68), Vec::with_capacity(68)];
            data_blocks[0].extend_from_slice(&data_bytes[..68]);
            data_blocks[1].extend_from_slice(&data_bytes[68..]);

            let mut code_bytes_left = true;
            let mut i = 0;
            while code_bytes_left {
                code_bytes_left = false;
                for block in data_blocks.iter() {
                    if let Some(byte) = block.get(i) {
                        code_bytes.push(*byte);
                        code_bytes_left = true;
                    }
                }
                i += 1;
            }

            // group 1 error correction + interleaving
            let mut ec_blocks = [Vec::with_capacity(18), Vec::with_capacity(18)];
            ec_blocks[0].extend_from_slice(&gf::ec_codewords(&f, &data_blocks[0], &poly));
            ec_blocks[1].extend_from_slice(&gf::ec_codewords(&f, &data_blocks[1], &poly));

            let mut code_bytes_left = true;
            let mut i = 0;
            while code_bytes_left {
                code_bytes_left = false;
                for block in ec_blocks.iter() {
                    if let Some(byte) = block.get(i) {
                        code_bytes.push(*byte);
                        code_bytes_left = true;
                    }
                }
                i += 1;
            }

            // add remainder bits
            code_bytes.push(0);

            code_bytes
        }

        pub fn build(&self) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
            if self.url.chars().any(|x| x >= '\u{00FF}') {
                return Err(String::from("url cannot be encoded as ISO 8859-1!"));
            }

            let data_bits = self.encode_chars_to_bits();

            // create error correction bits
            // create qr code template from image buf

            let data_bytes = bits_to_bytes(&data_bits);
            println!("Data Bytes: {:?}", &data_bytes[0..8]);

            let f = gf::Field::new();
            let poly = gf::gen_poly(&f, 18);

            let code_bytes = Self::split_and_interleave_bytes(data_bytes, &f, &poly);
            println!("Code Bytes: {:?}", &code_bytes[0..8]);

            // module placement in matrix
            let module_size = 1;
            let width = 41;
            let height = 41;
            let black = Rgba([0, 0, 0, 255]);
            let white = Rgba([255, 255, 255, 255]);
            let reserved = Rgba([0, 0, 255, 255]);

            let mut code = CodeImg::new(module_size, width, height, black, white, reserved);

            // place the data_bytes bits
            let mut x = width - 1;
            let mut y = height - 1;

            enum Move {
                Left,
                UpRight,
                DownRight,
            }

            let mut next_move = Move::Left;
            let mut prev_move = Move::UpRight;

            for byte in code_bytes {
                let mut mask = 0b10000000;
                for _ in 0..8 {
                    let color = (byte & mask != 0) == ((y + 1) % 2 == 0);
                    code.fill_module(x, y, color);

                    // print!("{} ", (byte & mask != 0));

                    match next_move {
                        Move::Left => {
                            // println!("Next move is left!");
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
                            // println!("Next move is up + right!");
                            if y != 0 && code.is_open(x + 1, y - 1) {
                                x += 1;
                                y -= 1;
                                next_move = Move::Left;
                                prev_move = Move::UpRight;
                            } else if y != 0 && code.is_open(x, y - 1) {
                                y -= 1;
                                next_move = Move::UpRight;
                                prev_move = Move::UpRight;
                            } else if y != 0 && code.is_open(x + 1, y - 2) {
                                x += 1;
                                y -= 2;
                                next_move = Move::Left;
                                prev_move = Move::UpRight;
                            } else if y != 0 && code.is_open(x + 1, y - 6) {
                                x += 1;
                                y -= 6;
                                next_move = Move::Left;
                                prev_move = Move::UpRight;
                            } else if x != 0 && code.is_open(x - 1, y) {
                                x -= 1;
                                next_move = Move::Left;
                                prev_move = Move::Left;
                            } else if x != 0 && code.is_open(x - 2, y) {
                                x -= 2;
                                next_move = Move::Left;
                                prev_move = Move::Left;
                            } else {
                                return Err(format!("No valid moves! at ({},{})", x, y));
                            }
                        }
                        Move::DownRight => {
                            // println!("Next move is down + right!");
                            if code.is_open(x + 1, y + 1) {
                                x += 1;
                                y += 1;
                                prev_move = Move::DownRight;
                            } else if code.is_open(x + 1, y + 2) {
                                x += 1;
                                y += 2;
                                prev_move = Move::DownRight;
                            } else if code.is_open(x + 1, y + 6) {
                                x += 1;
                                y += 6;
                                prev_move = Move::DownRight;
                            } else if x != 0 && code.is_open(x - 1, y) {
                                x -= 1;
                                prev_move = Move::Left;
                            } else if x != 0 && y != 0 && code.is_open(x - 1, y - 8) {
                                x -= 1;
                                y -= 8;
                                prev_move = Move::Left;
                            } else {
                                break;
                            }
                            next_move = Move::Left;
                        }
                    };
                    mask >>= 1;
                    // code.save("code.png").unwrap()
                }
                // break;
            }

            // generate format and version information
            let fmt = vec![1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1];

            for (i, bit) in fmt[..7].iter().enumerate() {
                let color = *bit == 1;
                code.fill_module(8, (height - 1) - i as u32, color);
                if code.is_reserved(i as u32, 8) {
                    code.fill_module(i as u32, 8, color);
                } else {
                    code.fill_module(i as u32 + 1, 8, color);
                }
            }
            for (i, bit) in fmt[7..].iter().enumerate() {
                let color = *bit == 1;

                code.fill_module((width - 8) + i as u32, 8, color);
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
