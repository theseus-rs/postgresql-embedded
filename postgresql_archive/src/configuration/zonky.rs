mod extractor;
mod matcher;
mod repository;

pub const URL: &str = "https://github.com/zonkyio/embedded-postgres-binaries";

pub use extractor::extract;
pub use matcher::matcher;
pub use repository::Zonky;
