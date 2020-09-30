use thiserror::Error;
#[derive(Error, Debug)]
pub enum InternalError {
    GroupReadAsData,
    DataReadAsGroup,
    OutOfBounds,
}
impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalError::GroupReadAsData => write!(f, "Group read as data."),
            InternalError::DataReadAsGroup => write!(f, "Data read as group."),
            InternalError::OutOfBounds => write!(f, "Out of bounds access."),
        }
    }
}

#[derive(Error, Debug)]
pub enum ParsingError {
    InvalidAlembicFile,
    UnsupportedAlembicFile,
}
impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::InvalidAlembicFile => write!(f, "Invalid Alembic File"),
            ParsingError::UnsupportedAlembicFile => write!(f, "Unsupported Alembic File"),
        }
    }
}

#[derive(Error, Debug)]
pub enum OgawaError {
    #[error("Internal error {0}")]
    InternalError(#[from] InternalError),

    #[error("Parsing error {0}")]
    ParsingError(#[from] ParsingError),

    #[error("I/O error {0}")]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
pub type Result<V, E = OgawaError> = ::std::result::Result<V, E>;
