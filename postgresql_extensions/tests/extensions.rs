use postgresql_extensions::get_available_extensions;

#[tokio::test]
async fn test_get_available_extensions() -> postgresql_extensions::Result<()> {
    let extensions = get_available_extensions().await?;
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
