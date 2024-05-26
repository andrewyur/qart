/// For all versions (1-40)
pub const fn side_len_of_version(version: u8) -> u32 {
    version as u32 * 4 + 17
}

/// # For byte mode
pub const MODE_IND: [u8; 4] = [0, 1, 0, 0];

/// # For byte mode, versions 1-9
pub const CHAR_CT_IND_LEN: usize = 8;

/// # For error correction level L, versions 1-7.
pub const fn required_data_bits(version: u8) -> usize {
    let data_bytes = [19, 34, 55, 80, 108, 136, 156];
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

/// # For error correction level L, versions 1-7.
pub const fn data_bytes_per_block(version: u8) -> usize {
    let data_bytes = [19, 32, 55, 80, 108, 68, 78];
    data_bytes[version as usize - 1]
}

/// # For error correction level L, versions 1-7.
pub const fn ec_bytes_per_block(version: u8) -> usize {
    let ec_bytes = [7, 10, 15, 20, 26, 18, 20];
    ec_bytes[version as usize - 1]
}

/// # For error correction level L, versions 1-7
pub const fn number_of_blocks(version: u8) -> usize {
    if version > 5 {
        2
    } else {
        1
    }
}

/// # For versions 1-7
/// Returns a tuple of the distance between columns/rows, and number of columns/rows.
/// Columns always start at 6, so the corresponding row/column locations for the tuple
/// `(16, 3)` would be: `[6, 22, 38]`
pub const fn pattern_locations(version: u8) -> (u32, u32) {
    if version < 7 {
        (version as u32 * 4 + 4, 2)
    } else {
        ((version as u32 - 7) * 2 + 16, 3)
    }
}

/// # For error correction level L, and mask pattern 1
pub const fn format_string(version: u8) -> [u8; 15] {
    [1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1]
}

/// # For version 7
pub const fn versions_string(version: u8) -> [u8; 18] {
    [0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0]
}
