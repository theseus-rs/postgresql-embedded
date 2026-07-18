pub mod registry;

/// Returns the target triple for selecting a PostgreSQL archive.
#[must_use]
pub const fn target() -> &'static str {
    target_triple::TARGET
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target() {
        assert_eq!(target_triple::TARGET, target());
    }
}
