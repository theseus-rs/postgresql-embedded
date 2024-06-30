use crate::Result;
use sha1::{Digest, Sha1};

/// Hashes the data using SHA1.
///
/// # Errors
/// * If the data cannot be hashed.
pub fn hash(data: &Vec<u8>) -> Result<String> {
    let mut hasher = Sha1::new();
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
        assert_eq!("1f3e1678e699640dfa5173d3a52b004f5e164d87", hash);
        Ok(())
    }
}
