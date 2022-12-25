use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VfsError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid absolute path: {path}")]
    InvalidAbsolutePath { path: String },

    #[error("Invalid file: {path}")]
    InvalidFile { path: String },

    #[error("Not supported")]
    NotSupported,

    /// Generic error context, used for adding context to an error (like a path)
    #[error("{context}, cause: {cause}")]
    WithContext {
        /// The context error message
        context: String,
        /// The underlying error
        #[source]
        cause: Box<VfsError>,
    },
}

pub type VfsResult<T> = std::result::Result<T, VfsError>;

pub(crate) trait VfsResultExt<T> {
    fn with_context<C, F>(self, f: F) -> VfsResult<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T> VfsResultExt<T> for VfsResult<T> {
    fn with_context<C, F>(self, context: F) -> VfsResult<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|error| VfsError::WithContext {
            context: context().to_string(),
            cause: Box::new(error),
        })
    }
}
