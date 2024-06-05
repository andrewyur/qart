// a fast implementation of the galois field of 256 and reed solomon encoding optimizations based on https://research.swtch.com/field
// operations are mod 285 to conform to QR code spec

use std::rc::Rc;

pub struct Field {
    log: [u8; 256],
    exp: [u8; 510],
}

impl Field {
    pub fn new() -> Self {
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
    pub fn exp(&self) -> &[u8] {
        &self.exp
    }

    // slice of log2 e values in the field
    pub fn log(&self) -> &[u8] {
        &self.log
    }
}

// returns a generator polynomial where n message encoding codewords are needed, using Field f
pub fn gen_poly(f: Rc<Field>, n: usize) -> Vec<u8> {
    let exp = f.exp();
    let log = f.log();

    // values in this are exponents of alpha coefficients
    let mut gen: Vec<u8> = Vec::with_capacity(n);
    gen.push(0);

    // curr must always be the same length as gen
    let mut curr = Vec::with_capacity(n);
    curr.push(0);

    while gen.len() < n + 1 {
        // multiply existing generator polynomial by (x - α^e)
        curr.copy_from_slice(&gen);

        let alpha_e = (gen.len() - 1) as u8;

        curr.iter_mut().for_each(|v| {
            *v = match (*v).checked_add(alpha_e) {
                Some(s) => s,
                None => {
                    ((alpha_e as u16 + *v as u16) % 256 + (alpha_e as u16 + *v as u16) / 256) // :(
                            as u8
                }
            }
        });

        for i in 0..(gen.len() - 1) {
            gen[i + 1] = log[(exp[curr[i] as usize] ^ exp[gen[i + 1] as usize]) as usize];
        }
        gen.push(curr[curr.len() - 1]);
        curr.push(0);
    }

    gen
}

// TODO: Make this extend the message vec with the bits instead of returning a new vec
// generates (gen.len() - 1) error correcting bytes for message mes, using Field f and generator polynomial gen
pub fn ec_codewords(f: Rc<Field>, mes: &[u8], gen: &[u8]) -> Vec<u8> {
    let exp = f.exp();
    let log = f.log();

    // vector with message in it, with room for remainder at the end
    let mut p: Vec<_> = Vec::with_capacity(mes.len() + gen.len() - 2);
    p.extend(mes.iter());
    p.extend(std::iter::repeat(0).take(gen.len() - 1));

    for i in 0..mes.len() {
        if p[i] == 0 {
            continue;
        }
        // k = pi / g0
        let k = log[p[i] as usize] as usize;
        // p -= k * g
        for (j, g) in gen.iter().enumerate() {
            p[i + j] ^= exp[k as usize + *g as usize];
        }
    }

    p[(mes.len())..].to_vec()
}
