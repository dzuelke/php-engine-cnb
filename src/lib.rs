mod build;
mod detect;
mod error;
mod layers;

pub use build::build;
pub use detect::detect;
pub use error::BuildpackError;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PhpBuildpackMetadata {
    pub php_url: String,
}
