use crate::{async_trait, OpenOptions, VfsError, VfsResult};
use futures_lite::{AsyncRead, AsyncSeek, AsyncWrite};
use std::{io::ErrorKind, pin::Pin};

pub trait VMetadata: Sync + Send {
    fn path(&self) -> &str;
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
    fn len(&self) -> u64;
    fn mtime(&self) -> u64;
}

pub trait VFile: AsyncRead + AsyncWrite + AsyncSeek {}
impl<T> VFile for T where T: AsyncRead + AsyncWrite + AsyncSeek {}

#[async_trait]
pub trait Vfs: Sync + Send {
    /// Returns true if the path points at an existing entity.
    async fn exists(&self, path: &str) -> VfsResult<bool> {
        match self.metadata(path).await {
            Ok(_) => Ok(true),
            Err(VfsError::IoError(err)) => match &err.kind() {
                ErrorKind::NotFound => Ok(false),
                _ => Err(VfsError::IoError(err)),
            },
            Err(e) => Err(e),
        }
    }

    async fn ls(
        &self,
        path: &str,
        skip_token: Option<String>,
    ) -> VfsResult<(Vec<Box<dyn VMetadata>>, Option<String>)>;

    async fn cp(&self, from: &str, to: &str) -> VfsResult<()>;
    async fn metadata(&self, path: &str) -> VfsResult<Box<dyn VMetadata>>;
    async fn mkdir(&self, path: &str) -> VfsResult<()>;
    async fn mv(&self, from: &str, to: &str) -> VfsResult<()>;
    async fn open(&self, path: &str, options: OpenOptions) -> VfsResult<Pin<Box<dyn VFile>>>;
    async fn rm(&self, path: &str) -> VfsResult<()>;
}
