mod extractor;
mod matcher;

pub const URL: &str = "https://github.com/theseus-rs/postgresql-binaries";

pub use extractor::extract;
pub use matcher::matcher;
