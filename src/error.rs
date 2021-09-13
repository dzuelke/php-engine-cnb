/// Buildpack Error Handling
#[derive(thiserror::Error, Debug)]
pub enum BuildpackError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid ProcessType name: {0}")]
    ProcessType(#[from] libcnb::data::launch::ProcessTypeError),
    #[error("TOML Error: {0}")]
    Toml(#[from] libcnb::TomlFileError),
}

impl From<BuildpackError> for libcnb::Error<BuildpackError> {
    fn from(error: BuildpackError) -> Self {
        Self::BuildpackError(error)
    }
}
