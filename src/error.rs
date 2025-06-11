#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("parse meta error: {0}")]
    ParseMetaError(String),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("flv error: invalid signature")]
    InvalidSignature,
}
