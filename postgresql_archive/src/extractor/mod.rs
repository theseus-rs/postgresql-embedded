mod model;
pub mod registry;
#[cfg(feature = "tar-gz")]
mod tar_gz_extractor;
#[cfg(feature = "tar-xz")]
mod tar_xz_extractor;
#[cfg(feature = "zip")]
mod zip_extractor;

pub use model::ExtractDirectories;
#[cfg(feature = "tar-gz")]
pub use tar_gz_extractor::extract as tar_gz_extract;
#[cfg(feature = "tar-xz")]
pub use tar_xz_extractor::extract as tar_xz_extract;
#[cfg(feature = "zip")]
pub use zip_extractor::extract as zip_extract;
