use crate::Result;
use sha3::{Digest, Sha3_256};

/// Hashes the data using SHA3-256.
///
/// # Errors
/// * If the data cannot be hashed.
pub fn hash(data: &Vec<u8>) -> Result<String> {
    let mut hasher = Sha3_256::new();
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
            "10a0812b3335c2f6de6dd195c77950e20dbd2e87ee95086db4e2fd42f1a78eed",
            hash
        );
        Ok(())
    }
}
