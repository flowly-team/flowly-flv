#[derive(Debug, thiserror::Error)]
pub enum Error<E = flowly::Void> {
    #[error("parse meta error: {0}")]
    ParseMetaError(String),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("flv error: invalid signature")]
    InvalidSignature,

    #[error(transparent)]
    Other(E),
}

impl<E: std::error::Error + Send + Sync + 'static> From<Error<E>> for std::io::Error {
    fn from(value: Error<E>) -> Self {
        match value {
            Error::IoError(error) => error,
            err => std::io::Error::new(std::io::ErrorKind::Other, err),
        }
    }
}
