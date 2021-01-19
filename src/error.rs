use thiserror::Error;

#[derive(Error, Debug)]
pub enum VfsError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type VfsResult<T> = std::result::Result<T, VfsError>;
