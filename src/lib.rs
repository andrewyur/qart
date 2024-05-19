pub mod qr {
    pub struct Code {
        url: String,
    }
    // target is assumed to be a url
    // error correction is assumed to be L
    // encoding is assumed to be binary (!TODO)
    // version number is decided by build (version 6)(!TODO)

    pub enum Error {
        EncodingError(String),
    }

    impl Code {
        // constructor, takes the information to be encoded in the code
        pub fn new(url: String) -> Self {
            Self { url }
        }

        pub fn build(&self) -> Result<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, Error> {
            if self.url.chars().any(|x| x >= '\u{00FF}') {
                return Err(Error::EncodingError(String::from(
                    "url cannot be encoded as ISO 8859-1!",
                )));
            }

            // see https://www.thonky.com/qr-code-tutorial
            let required_data_bits = 8 * 136;
            let mut data_bits: Vec<u8> = Vec::with_capacity(required_data_bits);
            let mut cursor = 0;

            // indicators
            let mode_indicator = vec![0, 1, 0, 0];
            let char_count_indicator = vec![0, 1, 0, 0, 0, 0, 1, 1, 0];

            mode_indicator.iter().for_each(|b| {
                data_bits[cursor] = *b;
                cursor += 1;
            });
            char_count_indicator.iter().for_each(|b| {
                data_bits[cursor] = *b;
                cursor += 1;
            });

            // encode data_bits
            self.url.as_bytes().iter().for_each(|b| {
                for j in (0..8).rev() {
                    data_bits[cursor] = (b >> j) & 1;
                    cursor += 1;
                }
            });

            // terminator
            if cursor + 1 > required_data_bits {
                return Err(Error::EncodingError(String::from(
                    "url does not fit into a v6 qr code!",
                )));
            }

            match required_data_bits - (cursor + 1) {
                x if x > 4 => (0..(x % 8)).for_each(|_| {
                    data_bits[cursor] = 0;
                    cursor += 1;
                }),
                x => (0..x).for_each(|_| {
                    data_bits[cursor] = 0;
                    cursor += 1;
                }),
            }

            //padding

            assert!((cursor + 1) % 8 != 0);

            let mut toggle = true;
            while cursor + 1 < required_data_bits {
                let byte = if toggle {
                    [1, 1, 1, 0, 1, 1, 0, 0]
                } else {
                    [0, 0, 0, 1, 0, 0, 0, 1]
                };

                toggle = !toggle;

                byte.into_iter().for_each(|b| {
                    data_bits[cursor] = b;
                    cursor += 1;
                })
            }

            // create error correction bits
            // create qr code template from image buf

            Ok(image::ImageBuffer::new(1, 1))
        }
    }
}
