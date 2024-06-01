/// For all versions (1-40)
pub const fn side_len_of_version(version: u32) -> u32 {
    assert!(version < 41);
    version * 4 + 17
}

pub const BYTE_MODE_IND: [u8; 4] = [0, 1, 0, 0];

pub const NUM_MODE_IND: [u8; 4] = [0, 0, 0, 1];

/// For byte mode, all versions (1-40)
pub const fn char_count_indicator_len_byte(version: u32) -> usize {
    assert!(version < 41);
    if version < 10 {
        8
    } else {
        16
    }
}

pub const fn char_count_indicator_len_num(version: u32) -> usize {
    assert!(version < 41);
    if version < 10 {
        10
    } else if version < 27 {
        12
    } else {
        14
    }
}

/// For error correction level L, all versions (1-40).
pub const fn required_data_bits(version: u32) -> usize {
    assert!(version < 41);
    let data_bytes = [
        19, 34, 55, 80, 108, 136, 156, 194, 232, 274, 324, 370, 428, 461, 523, 589, 647, 721, 795,
        861, 932, 1006, 1094, 1174, 1276, 1370, 1468, 1531, 1631, 1735, 1843, 1955, 2071, 2191,
        2306, 2434, 2566, 2702, 2812, 2956,
    ];
    data_bytes[version as usize - 1] * 8
}

/// Given number of padding bytes added, returns the padding byte to add next.
pub const fn pad_bytes(i: usize) -> [u8; 8] {
    if i % 2 == 0 {
        [1, 1, 1, 0, 1, 1, 0, 0]
    } else {
        [0, 0, 0, 1, 0, 0, 0, 1]
    }
}

// qr codes with more than 2 data groups are not covered.

/// For error correction level L, all versions (1-40).
pub const fn data_bytes_per_block(version: u32, group: u32) -> usize {
    assert!(group < 3);
    assert!(version < 41);
    let group1 = [
        19, 34, 55, 80, 108, 68, 78, 97, 116, 68, 81, 92, 107, 115, 87, 98, 107, 120, 113, 107,
        116, 111, 121, 117, 106, 114, 122, 117, 116, 115, 115, 115, 115, 115, 121, 121, 122, 122,
        117, 118,
    ];
    let group2 = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 69, 0, 93, 0, 116, 88, 99, 108, 121, 114, 108, 117, 112, 122,
        118, 107, 115, 123, 118, 117, 116, 116, 0, 116, 116, 122, 122, 123, 123, 118, 119,
    ];
    if group == 1 {
        group1[version as usize - 1]
    } else {
        group2[version as usize - 1]
    }
}

/// For error correction level L, all versions (1-40).
pub const fn ec_bytes_per_block(version: u32) -> usize {
    assert!(version < 41);
    let ec_bytes = [
        7, 10, 15, 20, 26, 18, 20, 24, 30, 18, 20, 24, 26, 30, 22, 24, 28, 30, 28, 28, 28, 28, 30,
        30, 26, 28, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    ];
    ec_bytes[version as usize - 1]
}

/// For error correction level L, all versions (1-40)
pub const fn number_of_blocks(version: u32, group: u32) -> usize {
    assert!(group < 3);
    assert!(version < 41);
    let group1 = [
        1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 4, 2, 4, 3, 5, 5, 1, 5, 3, 3, 4, 2, 4, 6, 8, 10, 8, 3, 7, 5,
        13, 17, 17, 13, 12, 6, 17, 4, 20, 19,
    ];
    let group2 = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 1, 1, 1, 5, 1, 4, 5, 4, 7, 5, 4, 4, 2, 4, 10, 7, 10,
        3, 0, 1, 6, 7, 14, 4, 18, 4, 6,
    ];
    if group == 1 {
        group1[version as usize - 1]
    } else {
        group2[version as usize - 1]
    }
}

/// For error correction level L, all versions (1-40)
pub const fn number_of_groups(version: u32) -> usize {
    assert!(version < 41);
    if number_of_blocks(version, 2) == 0 {
        1
    } else {
        2
    }
}

// /// For error correction level L, all versions (1-40)
// pub const fn total_number_of_bytes(version: u32) -> usize {
//     assert!(version < 41);
//     number_of_blocks(version, 1) * (data_bytes_per_block(version, 1) + ec_bytes_per_block(version))
//         + if number_of_groups(version) == 2 {
//             number_of_blocks(version, 2)
//                 * (data_bytes_per_block(version, 2) + ec_bytes_per_block(version))
//         } else {
//             0
//         }
// }

// this is bad
/// For all versions (1-40)
pub fn pattern_locations(version: u32) -> Vec<u32> {
    assert!(version < 41);
    let mut pattern_locations = vec![
        vec![],
        vec![6, 18],
        vec![6, 22],
        vec![6, 26],
        vec![6, 30],
        vec![6, 34],
        vec![6, 22, 38],
        vec![6, 24, 42],
        vec![6, 26, 46],
        vec![6, 28, 50],
        vec![6, 30, 54],
        vec![6, 32, 58],
        vec![6, 34, 62],
        vec![6, 26, 46, 66],
        vec![6, 26, 48, 70],
        vec![6, 26, 50, 74],
        vec![6, 30, 54, 78],
        vec![6, 30, 56, 82],
        vec![6, 30, 58, 86],
        vec![6, 34, 62, 90],
        vec![6, 28, 50, 72, 94],
        vec![6, 26, 50, 74, 98],
        vec![6, 30, 54, 78, 102],
        vec![6, 28, 54, 80, 106],
        vec![6, 32, 58, 84, 110],
        vec![6, 30, 58, 86, 114],
        vec![6, 34, 62, 90, 118],
        vec![6, 26, 50, 74, 98, 122],
        vec![6, 30, 54, 78, 102, 126],
        vec![6, 26, 52, 78, 104, 130],
        vec![6, 30, 56, 82, 108, 134],
        vec![6, 34, 60, 86, 112, 138],
        vec![6, 30, 58, 86, 114, 142],
        vec![6, 34, 62, 90, 118, 146],
        vec![6, 30, 54, 78, 102, 126, 150],
        vec![6, 24, 50, 76, 102, 128, 154],
        vec![6, 28, 54, 80, 106, 132, 158],
        vec![6, 32, 58, 84, 110, 136, 162],
        vec![6, 26, 54, 82, 110, 138, 166],
        vec![6, 30, 58, 86, 114, 142, 170],
    ];
    return pattern_locations.remove(version as usize - 1);
}

/// # For error correction level L, and mask pattern 1
pub const FORMAT_STRING: [u8; 15] = [1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1];

/// For versions 7-40
pub const fn versions_string(version: u32) -> [u8; 18] {
    assert!(version < 41);
    let version_strings = [
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
    version_strings[version as usize - 7]
}

/// For error correction level L, all versions (1-40)
pub const fn char_capacity(version: u32) -> usize {
    let capacities = [
        17, 32, 53, 78, 106, 134, 154, 192, 230, 271, 321, 367, 425, 458, 520, 586, 644, 718, 792,
        858, 929, 1003, 1091, 1171, 1273, 1367, 1465, 1528, 1628, 1732, 1840, 1952, 2068, 2188,
        2303, 2431, 2563, 2699, 2809, 2953,
    ];
    capacities[version as usize - 1]
}

/// For error correction level L, all versions (1-40)
pub const fn total_blocks(version: u32) -> usize {
    number_of_blocks(version, 1) + number_of_blocks(version, 2)
}
