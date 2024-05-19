fn main() {
    // let img_width = 300;
    // let img_height = 300;

    // let mut img_buf = image::ImageBuffer::new(img_width, img_height);

    // for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
    //     *pixel = image::Rgb([x as u8, y as u8, 0])
    // }

    // img_buf.save("image.png").unwrap();

    let f = gf::Field::new();

    println!("{}", f.mul_f(232, 2));
}
// a fast implementation of the galois field of 256 and reed solomon encoding optimizations based on https://research.swtch.com/field
// operations are mod 285 to conform to QR code spec
mod gf {
    pub struct Field {
        log: [u16; 256],
        exp: [u16; 510],
    }

    impl Field {
        pub fn new() -> Self {
            // eventually, this should cache these slices to a file, and load them if they are present
            let mut log = [0; 256];
            let mut exp = [0; 510];

            //generator is 2
            let mut x: u16 = 1;
            for i in 0..255 {
                exp[i] = x;
                exp[i + 255] = x;
                log[x as usize] = i as u16;
                x <<= 1;
                if x & 0x100 != 0 {
                    x ^= 285
                }
            }
            log[0] = 255;

            Self { log, exp }
        }

        // slice of 2^e values in the field
        pub fn exp(&self) -> [u16; 510] {
            self.exp.to_owned()
        }

        // slice of log2 e values in the field
        pub fn log(&self) -> [u16; 256] {
            self.log.to_owned()
        }

        // multiplication via addition of logarithms
        pub fn mul_f(&self, x: u16, y: u16) -> u16 {
            if x == 0 || y == 0 {
                return 0;
            }

            // this is why exp is 510 long, so we dont have to do % 255
            self.exp[(self.log[x as usize] + self.log[y as usize]) as usize]
        }
    }

    // generates n message encoding bits for message slice mes, using Field f
    // assumes n is less than 255
    fn encode(f: Field, mes: &[u16], n: usize) {
        let m = mes.len();

        let exp = f.exp();
        let log = f.log();

        //
        //  TODO: there should be a faster and cleaner way to do this
        //

        // values in this are exponents of alpha coefficients
        let mut gen: Vec<Option<u16>> = Vec::with_capacity(n);
        gen.push(Some(0));

        // curr must always be in sync with gen
        let mut curr = Vec::with_capacity(n);
        curr.push(None);

        while gen.len() < n {
            curr.push(None);
            curr[1..].copy_from_slice(&gen);
            // multiply existing generator polynomial by (x - α^e)

            let alpha_e = gen.len() as u16 - 1;

            gen.push(None);

            curr.iter_mut().for_each(|v| match v {
                Some(e) => *e += alpha_e,
                None => (),
            });

            for (gen_e, curr_e) in std::iter::zip(gen.iter_mut(), curr.iter_mut()) {
                match (*gen_e, *curr_e) {
                    (_, None) => continue,
                    (None, _) => *gen_e = *curr_e,
                    (g_e, c_e) => {
                        *gen_e = Some(
                            log[(exp[g_e.unwrap() as usize] ^ exp[c_e.unwrap() as usize]) as usize],
                        );
                    }
                }
            }
        }
    }
}
