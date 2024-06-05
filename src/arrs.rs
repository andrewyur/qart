// type aliases to distinguish between arrays of bits and arrays of bytes, and conversion + qol methods

pub type ByteArr = Vec<u8>;
pub trait ByteArrMethods {
    fn bit_at(&self, index: usize) -> u8;
    fn to_bits(&self) -> Vec<u8>;
}
impl ByteArrMethods for ByteArr {
    fn bit_at(&self, index: usize) -> u8 {
        (self[index / 8] >> (7 - (index & 7))) & 1
    }
    fn to_bits(&self) -> Vec<u8> {
        self.iter()
            .fold(Vec::with_capacity(self.len() * 8), |mut v, byte| {
                let mut mask = 0b10000000;
                for _ in 0..8 {
                    let bit = ((byte & mask) != 0) as u8;
                    v.push(bit);
                    mask >>= 1;
                }
                v
            })
    }
}

pub type BitArr = Vec<Bit>;
pub trait BitArrMethods {
    fn extend_bits(&mut self, bits: &[u8], role: Role);
    fn extend_bytes(&mut self, bytes: &[u8], role: Role);
    fn to_byte_arr(&self) -> ByteArr;
}
impl BitArrMethods for BitArr {
    fn extend_bits(&mut self, bits: &[u8], role: Role) {
        let iter = bits.iter().map(|b| Bit { val: *b == 1, role });
        self.extend(iter);
    }
    fn extend_bytes(&mut self, bytes: &[u8], role: Role) {
        bytes.iter().for_each(|byte| {
            let mut mask = 0b10000000;
            for _ in 0..8 {
                let bit = ((byte & mask) != 0) as u8;
                self.push(Bit {
                    val: bit == 1,
                    role,
                });
                mask >>= 1;
            }
        })
    }
    fn to_byte_arr(&self) -> ByteArr {
        (0..(self.len() / 8)).fold(Vec::with_capacity(self.len() / 8), |mut v, i| {
            let i = i * 8;
            let mut s: u8 = 0;
            let mut mask = 0b10000000;
            for j in 0..8 {
                let val = self[i + j].val as u8;
                s += val * mask;
                mask >>= 1;
            }
            v.push(s);
            v
        })
    }
}

#[derive(Clone)]
pub struct Bit {
    // true = 1 = black
    pub val: bool,
    pub role: Role,
}

// TODO: change this back to a can_edit bool
#[derive(Clone, Copy)]
pub enum Role {
    Data,
    EC,
    Num,
}

impl Bit {
    pub fn can_edit(&self) -> bool {
        match self.role {
            Role::Data => false,
            _ => true,
        }
    }
    pub fn is_num(&self) -> bool {
        match self.role {
            Role::Num => true,
            _ => false,
        }
    }
}
