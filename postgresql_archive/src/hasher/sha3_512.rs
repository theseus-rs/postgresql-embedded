use crate::Result;
use sha3::{Digest, Sha3_512};

/// Hashes the data using SHA3-512.
///
/// # Errors
/// * If the data cannot be hashed.
pub fn hash(data: &Vec<u8>) -> Result<String> {
    let mut hasher = Sha3_512::new();
    hasher.update(data);
    let hash = hex::encode(hasher.finalize());
    Ok(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() -> Result<()> {
        let data = vec![4, 2];
        let hash = hash(&data)?;
        assert_eq!(
            "4bcb3a87557684ff56272f6bc7f542d728d1b953d8b0beb94ffdd97d9ba872550629c9eb98357060c7dce1786f91e6af948eb1ae21ec304f558a4651ff2b134f",
            hash
        );
        Ok(())
    }
}
