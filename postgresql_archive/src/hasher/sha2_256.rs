use crate::Result;
use sha2::{Digest, Sha256};

/// Hashes the data using SHA2-256.
///
/// # Errors
/// * If the data cannot be hashed.
pub fn hash(data: &Vec<u8>) -> Result<String> {
    let mut hasher = Sha256::new();
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
            "b7586d310e5efb1b7d10a917ba5af403adbf54f4f77fe7fdcb4880a95dac7e7e",
            hash
        );
        Ok(())
    }
}
