/// Compute the checksum of the provided bytes
pub fn checksum(data: &[u8]) -> u8 {
    data.iter().fold(0, |acc, &byte| acc.wrapping_add(byte))
}

/// Encrypts the provided bytes by xoring them with the provided u8 key.
pub fn encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|byte| byte ^ key).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        assert_eq!(checksum(&[0]), 0);
        assert_eq!(checksum(&[1]), 1);
        assert_eq!(checksum(&[255]), 255);

        assert_eq!(checksum(&[0, 0]), 0);
        assert_eq!(checksum(&[0, 1]), 1);
        assert_eq!(checksum(&[1, 0]), 1);
        assert_eq!(checksum(&[1, 1]), 2);
        assert_eq!(checksum(&[255, 1]), 0);
        assert_eq!(checksum(&[255, 255]), 254);

        assert_eq!(checksum(&[255, 255, 1]), 255);
        assert_eq!(checksum(&[255, 255, 2]), 0);
    }

    #[test]
    fn test_encrypt() {
        assert_eq!(encrypt(&[1, 2, 3], 0), [1, 2, 3]);
        assert_eq!(encrypt(&[1, 2, 3], 1), [0, 3, 2]);
        assert_eq!(encrypt(&[1, 2, 3], 0xf0), [0xf1, 0xf2, 0xf3]);
    }
}
