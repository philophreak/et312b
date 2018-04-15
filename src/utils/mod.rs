/// Compute the checksum of the provided bytes
pub fn checksum(data: &[u8]) -> u8 {
    data.iter().fold(0, |acc, &byte| acc.wrapping_add(byte))
}

/// Encrypts the provided bytes by xoring them with the provided u8 key.
pub fn encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|byte| byte ^ key).collect()
}
