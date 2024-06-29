use crate::Result;
use sha2::{Digest, Sha512};

/// Hashes the data using SHA2-512.
///
/// # Errors
/// * If the data cannot be hashed.
pub fn hash(data: &Vec<u8>) -> Result<String> {
    let mut hasher = Sha512::new();
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
            "7df6418d1791a6fe80e726319f16f107534a663346f99e0d155e359a54f6c74391e2f3be19c995c3c903926d348bd86c339bd982e10f09aa776e4ff85d36387a",
            hash
        );
        Ok(())
    }
}
