use crate::Result;
use blake2::{Blake2s256, Digest};

/// Hashes the data using blake2s-256.
///
/// # Arguments
/// * `data` - The data to hash.
///
/// # Returns
/// * The hash of the data.
///
/// # Errors
/// * If the data cannot be hashed.
pub fn hash(data: &Vec<u8>) -> Result<String> {
    let mut hasher = Blake2s256::new();
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
            "d2e78507a899636d20314690e1683fd2116dd438b502645a8aa8f79fd579fb70",
            hash
        );
        Ok(())
    }
}
