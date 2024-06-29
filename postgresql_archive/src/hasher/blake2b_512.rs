use crate::Result;
use blake2::{Blake2b512, Digest};

/// Hashes the data using blake2b-512.
///
/// # Errors
/// * If the data cannot be hashed.
pub fn hash(data: &Vec<u8>) -> Result<String> {
    let mut hasher = Blake2b512::new();
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
            "e487ff1a06742b6054c76387d7a0bf9e0f62964358b850d80d9f88071508ef855e745a8ba67617f850cf563b20f4ec0d5bd8233b2e85eb0ba4f31a14075fb3d9",
            hash
        );
        Ok(())
    }
}
