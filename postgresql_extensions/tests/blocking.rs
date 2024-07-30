#[cfg(feature = "blocking")]
use postgresql_extensions::blocking::get_available_extensions;
#[cfg(feature = "blocking")]
use test_log::test;

#[cfg(feature = "blocking")]
#[test]
fn test_get_available_extensions() -> postgresql_extensions::Result<()> {
    let extensions = get_available_extensions()?;
    #[cfg(feature = "steampipe")]
    assert!(extensions
        .iter()
        .any(|extension| extension.namespace() == "steampipe"));
    #[cfg(feature = "tensor-chord")]
    assert!(extensions
        .iter()
        .any(|extension| extension.namespace() == "tensor-chord"));
    Ok(())
}
