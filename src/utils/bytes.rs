pub fn to_little_endian(value: u64) -> [u8; 8] {
    value.to_le_bytes()
}