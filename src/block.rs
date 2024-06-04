// This struct comes from the bitblock struct in the other implementations, allows for editing of ec modules without corrupting data
use crate::{
    arrs::{BitArr, BitArrMethods, ByteArr, ByteArrMethods},
    gf::{self, Field},
};
use std::rc::Rc;

pub struct Block {
    num_data_bytes: usize,
    block_bytes: ByteArr,
    basis: Vec<Option<ByteArr>>,
    used: Vec<Option<ByteArr>>,
    numeric_data_start: usize,
    numeric_data_end: usize,
}

impl Block {
    pub fn new(num_data_bytes: usize, field: Rc<Field>, block_bits: BitArr) -> Self {
        let in_block_bytes = block_bits.to_byte_arr();

        let num_ec_bytes = in_block_bytes.len() - num_data_bytes;

        let poly = gf::gen_poly(Rc::clone(&field), num_ec_bytes);

        let mut block_bytes = Vec::with_capacity(num_ec_bytes + num_data_bytes);
        block_bytes.extend_from_slice(&in_block_bytes[..num_data_bytes]);
        block_bytes.extend_from_slice(&in_block_bytes[num_data_bytes..]);

        let mut basis =
            vec![Some(Vec::with_capacity(num_data_bytes + num_ec_bytes)); num_data_bytes * 8];
        let used = vec![None; num_data_bytes * 8];

        for (index, mask) in basis.iter_mut().enumerate() {
            mask.as_mut()
                .unwrap()
                .extend(std::iter::repeat_with(|| 0).take(num_data_bytes));

            mask.as_mut().unwrap()[index / 8] = 1 << (7 - (index & 7));

            let ec_bytes = gf::ec_codewords(Rc::clone(&field), &mask.as_ref().unwrap(), &poly);
            mask.as_mut().unwrap().extend_from_slice(&ec_bytes);
        }

        let mut numeric_data_start = 0;
        let mut numeric_data_end = 0;
        let mut data_found = false;
        for index in 0..basis.len() {
            if !block_bits[index].can_edit() {
                basis[index].take();
            }
            if block_bits[index].is_num() {
                if data_found {
                    assert!(block_bits[index - 1].is_num());
                    numeric_data_end += 1;
                } else {
                    numeric_data_start = index;
                    numeric_data_end = index + 1;
                    data_found = true
                }
            }
        }

        Self {
            num_data_bytes,
            block_bytes,
            basis,
            used,
            numeric_data_start,
            numeric_data_end,
        }
    }

    pub fn set(&mut self, index: usize, val: u8) -> bool {
        let mut found: Option<Vec<u8>> = None;
        let mut found_index = 0;

        // finds a row in the matrix where the bit at the index is set, and zeroes out that bit at all the other rows
        for (j, row_opt) in self.basis.iter_mut().enumerate() {
            if let Some(row) = row_opt {
                if row.bit_at(index) == 0 {
                    continue;
                }
                if let Some(targ) = found.as_ref() {
                    for k in 0..row.len() {
                        row[k] ^= targ[k];
                    }
                } else {
                    found = row_opt.take();
                    found_index = j;
                }
            }
        }

        if found.is_none() {
            return false;
        }

        // zeroes out that bit in the used rows too
        for row_opt in self.used.iter_mut() {
            if let Some(row) = row_opt {
                if row.bit_at(index) == 1 {
                    let targ = found.as_ref().unwrap();
                    for k in 0..row.len() {
                        row[k] ^= targ[k];
                    }
                }
            }
        }

        // so now we have found a row where the bit at index is 1, and then cut that bit from all the other rows
        // now we apply that row to the block if we need to, and since we took that row out of basis,
        // that bit cannot be changed again
        if self.block_bytes.bit_at(index) != val {
            let targ = found.as_ref().unwrap();
            for j in 0..targ.len() {
                self.block_bytes[j] ^= targ[j];
            }
        }

        // move the row into used
        self.used[found_index] = found;

        true
    }

    pub fn reset(&mut self, index: usize) {
        // if the bit has been set already, and it needs to be reset because it has caused an invalid number generation,
        // this function resets the bit by finding the row that was used to set it, and reversing the operation

        // if the bit is already set to the desired value, no need to do anything
        if self.block_bytes.bit_at(index) == 0 {
            return;
        }

        let mut found = false;
        for row_opt in self.used.iter() {
            if let Some(row) = row_opt {
                if row.bit_at(index) != 0 {
                    for k in 0..row.len() {
                        self.block_bytes[k] ^= row[k];
                    }
                    // row_opt.take();
                    found = true;
                    break;
                }
            }
        }
        if found {
            return;
        } else {
            assert!(self.set(index, 0));
        }
    }

    pub fn iter_nums<'b>(&'b self) -> impl Iterator<Item = (usize, u8)> + 'b {
        BlockIter::new(self, self.numeric_data_start, self.numeric_data_end)
            .enumerate()
            .map(move |(i, bit)| (self.numeric_data_start + i, bit))
    }

    pub fn iter_data_ec<'b>(
        &'b self,
    ) -> (
        Box<dyn Iterator<Item = (usize, u8)> + 'b>,
        Box<dyn Iterator<Item = (usize, u8)> + 'b>,
    ) {
        (
            Box::new(BlockIter::new(self, 0, self.num_data_bytes * 8).enumerate()),
            Box::new(
                BlockIter::new(self, self.num_data_bytes * 8, self.block_bytes.len() * 8)
                    .enumerate()
                    .map(move |(i, bit)| (self.num_data_bytes * 8 + i, bit)),
            ),
        )
    }

    pub fn ret(self) -> ByteArr {
        self.block_bytes.to_bits()
    }

    pub fn get(&self, index: usize) -> u8 {
        self.block_bytes.bit_at(index)
    }

    pub fn debug(&self) {
        println!("{:02X?}", self.block_bytes);
    }
}

// bit iterator for block
pub struct BlockIter<'a> {
    block: &'a Block,
    pos: usize,
    end: usize,
}

impl<'a> BlockIter<'a> {
    fn new(block: &'a Block, start: usize, end: usize) -> Self {
        BlockIter {
            block,
            pos: start,
            end,
        }
    }
}

impl Iterator for BlockIter<'_> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.end {
            self.pos += 1;
            Some(self.block.get(self.pos - 1))
        } else {
            None
        }
    }
}
