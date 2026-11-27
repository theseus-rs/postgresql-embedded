#[cfg(feature = "github")]
pub mod github;
pub mod model;
pub mod registry;

pub use model::{Archive, Repository};
