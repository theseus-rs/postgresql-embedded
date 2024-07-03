#[cfg(feature = "github")]
pub mod github;
#[cfg(feature = "maven")]
pub mod maven;
pub mod model;
pub mod registry;

pub use model::{Archive, Repository};
