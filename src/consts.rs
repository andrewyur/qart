// constants values used when creating qr codes

use anyhow::anyhow;

#[derive(Copy, Clone)]
pub struct Version(u8);

impl Version {
    pub fn new(v: u8) -> anyhow::Result<Self> {
        if 1 <= v && v <= 40 {
            Ok(Self(v))
        } else {
            Err(anyhow!("Version number must be between 1 and 40"))
        }
    }

    #[inline(always)]
    pub const fn idx(self) -> usize {
        (self.0 - 1) as usize
    }

    #[inline(always)]
    pub const fn get(self) -> u8 {
        self.0
    }
}


/// For all versions (1-40)
pub const fn side_len_of_version(v: Version) -> u32 {
    (v.get() * 4 + 17) as u32
}

pub const BYTE_MODE_IND: [u8; 4] = [0, 1, 0, 0];

pub const NUM_MODE_IND: [u8; 4] = [0, 0, 0, 1];

/// For byte mode, all versions (1-40)
pub const fn char_count_indicator_len_byte(v: Version) -> usize {
    if v.get() < 10 {
        8
    } else {
        16
    }
}

pub const fn char_count_indicator_len_num(v: Version) -> usize {
    if v.get() < 10 {
        10
    } else if v.get() < 27 {
        12
    } else {
        14
    }
}

const DATA_BYTES: [usize; 40] = [
    19, 34, 55, 80, 108, 136, 156, 194, 232, 274, 324, 370, 428, 461, 523, 589, 647, 721, 795,
    861, 932, 1006, 1094, 1174, 1276, 1370, 1468, 1531, 1631, 1735, 1843, 1955, 2071, 2191,
    2306, 2434, 2566, 2702, 2812, 2956,
];

/// For error correction level L, all versions (1-40).
pub const fn required_data_bits(v: Version) -> usize {
    DATA_BYTES[v.idx()] * 8
}

// qr codes with more than 2 data groups are not covered.
const GROUP_1_BYTES: [usize; 40] = [
    19, 34, 55, 80, 108, 68, 78, 97, 116, 68, 81, 92, 107, 115, 87, 98, 107, 120, 113, 107,
    116, 111, 121, 117, 106, 114, 122, 117, 116, 115, 115, 115, 115, 115, 121, 121, 122, 122,
    117, 118,
];
const GROUP_2_BYTES: [usize; 40] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 69, 0, 93, 0, 116, 88, 99, 108, 121, 114, 108, 117, 112, 122,
    118, 107, 115, 123, 118, 117, 116, 116, 0, 116, 116, 122, 122, 123, 123, 118, 119,
];

/// For error correction level L, all versions (1-40).
pub const fn data_bytes_per_block(v: Version, group: u32) -> usize {
    assert!(group < 3);
    if group == 1 {
        GROUP_1_BYTES[v.idx()]
    } else {
        GROUP_2_BYTES[v.idx()]
    }
}

const EC_BYTES: [usize; 40] = [
    7, 10, 15, 20, 26, 18, 20, 24, 30, 18, 20, 24, 26, 30, 22, 24, 28, 30, 28, 28, 28, 28, 30,
    30, 26, 28, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
];

/// For error correction level L, all versions (1-40).
pub const fn ec_bytes_per_block(v: Version) -> usize {
    EC_BYTES[v.idx()]
}

const GROUP_1_BLOCKS: [usize; 40] = [
    1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 4, 2, 4, 3, 5, 5, 1, 5, 3, 3, 4, 2, 4, 6, 8, 10, 8, 3, 7, 5,
    13, 17, 17, 13, 12, 6, 17, 4, 20, 19,
];
const GROUP_2_BLOCKS: [usize; 40] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 1, 1, 1, 5, 1, 4, 5, 4, 7, 5, 4, 4, 2, 4, 10, 7, 10,
    3, 0, 1, 6, 7, 14, 4, 18, 4, 6,
];
/// For error correction level L, all versions (1-40)
pub const fn number_of_blocks(v: Version, group: u32) -> usize {
    assert!(group < 3);
    if group == 1 {
        GROUP_1_BLOCKS[v.idx()]
    } else {
        GROUP_2_BLOCKS[v.idx()]
    }
}

/// For error correction level L, all versions (1-40)
pub const fn number_of_groups(v: Version) -> usize {
    if number_of_blocks(v, 2) == 0 {
        1
    } else {
        2
    }
}

const PATTERN_LOCATIONS: [[u32; 7]; 40] = [
    [0, 0, 0, 0, 0, 0, 0],
    [6, 18, 0, 0, 0, 0, 0],
    [6, 22, 0, 0, 0, 0, 0],
    [6, 26, 0, 0, 0, 0, 0],
    [6, 30, 0, 0, 0, 0, 0],
    [6, 34, 0, 0, 0, 0, 0],
    [6, 22, 38, 0, 0, 0, 0],
    [6, 24, 42, 0, 0, 0, 0],
    [6, 26, 46, 0, 0, 0, 0],
    [6, 28, 50, 0, 0, 0, 0],
    [6, 30, 54, 0, 0, 0, 0],
    [6, 32, 58, 0, 0, 0, 0],
    [6, 34, 62, 0, 0, 0, 0],
    [6, 26, 46, 66, 0, 0, 0],
    [6, 26, 48, 70, 0, 0, 0],
    [6, 26, 50, 74, 0, 0, 0],
    [6, 30, 54, 78, 0, 0, 0],
    [6, 30, 56, 82, 0, 0, 0],
    [6, 30, 58, 86, 0, 0, 0],
    [6, 34, 62, 90, 0, 0, 0],
    [6, 28, 50, 72, 94, 0, 0],
    [6, 26, 50, 74, 98, 0, 0],
    [6, 30, 54, 78, 102, 0, 0],
    [6, 28, 54, 80, 106, 0, 0],
    [6, 32, 58, 84, 110, 0, 0],
    [6, 30, 58, 86, 114, 0, 0],
    [6, 34, 62, 90, 118, 0, 0],
    [6, 26, 50, 74, 98, 122, 0],
    [6, 30, 54, 78, 102, 126, 0],
    [6, 26, 52, 78, 104, 130, 0],
    [6, 30, 56, 82, 108, 134, 0],
    [6, 34, 60, 86, 112, 138, 0],
    [6, 30, 58, 86, 114, 142, 0],
    [6, 34, 62, 90, 118, 146, 0],
    [6, 30, 54, 78, 102, 126, 150],
    [6, 24, 50, 76, 102, 128, 154],
    [6, 28, 54, 80, 106, 132, 158],
    [6, 32, 58, 84, 110, 136, 162],
    [6, 26, 54, 82, 110, 138, 166],
    [6, 30, 58, 86, 114, 142, 170],
];

/// For all versions (1-40)
pub fn pattern_locations(v: Version) -> &'static [u32] {
    let row = &PATTERN_LOCATIONS[v.idx()];
    
    let mut n = 0;
    while n < 7 && row[n] != 0 {
        n += 1;
    }
    &row[..n]
}

/// # For error correction level L, and mask pattern 1
pub const FORMAT_STRING: [u8; 15] = [1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1];

const VERSION_STRINGS: [[u8; 18]; 34] = [
        [0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0],
        [0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1],
        [0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1],
        [0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0],
        [0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0],
        [0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1],
        [0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1],
        [0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1],
        [0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1],
        [0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0],
        [0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0],
        [0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1],
        [0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1],
        [0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0],
        [0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0],
        [0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1],
        [0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1],
        [0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0],
        [0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0],
        [0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1],
        [0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1],
        [0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0],
        [1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0],
        [1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1],
        [1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1],
        [1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0],
        [1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0],
        [1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1],
    ];

/// For versions 7-40
pub const fn versions_string(v: Version) -> [u8; 18] {
    VERSION_STRINGS[v.idx() - 6]
}

/// For error correction level L, all versions (1-40)
pub const fn total_blocks(v: Version) -> usize {
    number_of_blocks(v, 1) + number_of_blocks(v, 2)
}

const CHAR_CAPACITIES: [usize; 40] = [
    41, 77, 127, 187, 255, 322, 370, 461, 552, 652, 772, 883, 1022, 1101, 1250, 1408, 1548,
    1725, 1903, 2061, 2232, 2409, 2620, 2812, 3057, 3283, 3517, 3669, 3909, 4158, 4417, 4686,
    4965, 5253, 5529, 5836, 6153, 6479, 6743, 7089,
];

/// For error correction level L, all versions (1-40)
pub const fn numeric_char_capacity(v: Version) -> usize {
    CHAR_CAPACITIES[v.idx()]
}
