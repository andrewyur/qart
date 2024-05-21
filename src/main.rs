fn main() {
    // let img_width = 300;
    // let img_height = 300;

    // let mut img_buf = image::ImageBuffer::new(img_width, img_height);

    // for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
    //     *pixel = image::Rgb([x as u8, y as u8, 0])
    // }

    // img_buf.save("image.png").unwrap();

    let f = gf::Field::new();

    println!("{:?}", gf::gen_poly(f, 10));
}
// a fast implementation of the galois field of 256 and reed solomon encoding optimizations based on https://research.swtch.com/field
// operations are mod 285 to conform to QR code spec
mod gf {
    pub struct Field {
        log: [u8; 256],
        exp: [u8; 510],
    }

    impl Field {
        pub fn new() -> Self {
            // eventually, this should cache these slices to a file, and load them if they are present
            let mut log = [0; 256];
            let mut exp = [0; 510];

            //generator is 2
            let mut x: u16 = 1;
            for i in 0..255 {
                exp[i] = x as u8;
                exp[i + 255] = x as u8;
                log[x as usize] = i as u8;
                x <<= 1;
                if x & 0x100 != 0 {
                    x ^= 285
                }
            }
            log[0] = 255;

            Self { log, exp }
        }

        // slice of 2^e values in the field
        pub fn exp(&self) -> [u8; 510] {
            self.exp.to_owned()
        }

        // slice of log2 e values in the field
        pub fn log(&self) -> [u8; 256] {
            self.log.to_owned()
        }

        // multiplication via addition of logarithms
        pub fn mul_f(&self, x: u8, y: u8) -> u8 {
            if x == 0 || y == 0 {
                return 0;
            }

            // this is why exp is 510 long, so we dont have to do % 255
            self.exp[(self.log[x as usize] + self.log[y as usize]) as usize]
        }
    }

    // generates a generator polynomial where n message encoding bits are needed, using Field f
    // no way to check if generator polynomial is correct past n = 254
    pub fn gen_poly(f: Field, n: usize) -> Vec<u8> {
        let exp = f.exp();
        let log = f.log();

        // values in this are exponents of alpha coefficients
        let mut gen: Vec<u8> = Vec::with_capacity(n);
        gen.push(0);

        // curr must always be in sync with gen
        let mut curr = Vec::with_capacity(n);
        curr.push(0);

        while gen.len() < n + 1 {
            // multiply existing generator polynomial by (x - α^e)
            curr.copy_from_slice(&gen);

            let alpha_e = gen.len() as u8 - 1;

            curr.iter_mut().for_each(|v| {
                *v = match (*v).checked_add(alpha_e) {
                    Some(s) => s,
                    None => {
                        ((alpha_e as u16 + *v as u16) % 256 + (alpha_e as u16 + *v as u16) / 256)
                            as u8
                    } // :(
                }
            });

            for i in 0..(gen.len() - 1) {
                gen[i + 1] = log[(exp[curr[i] as usize] ^ exp[gen[i + 1] as usize]) as usize];
            }
            gen.push(curr[curr.len() - 1]);
            curr.push(0);
        }

        // translate exponents into numbers
        gen.iter().map(|e| exp[*e as usize]).collect()
    }

    // generates (gen.len() - 1) error correcting bits for message mes, using Field f and generator polynomial gen
    fn EC_codewords(f: Field, mes: &[u8], gen: &[u16]) {
        let ec_codewords: Vec<u16> = vec![];
    }
}
