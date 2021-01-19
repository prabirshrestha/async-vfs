use crate::{OpenOptions, VfsResult};
use async_std::io::{Read, Seek, Write};
use async_trait::async_trait;

pub trait VMetadata {
    fn path(&self) -> &str;
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
    fn len(&self) -> u64;
}

pub trait VFile: Read + Write + Seek {}

#[async_trait]
pub trait Vfs {
    async fn exists(&self, path: &str) -> VfsResult<bool>;
    async fn metadata(&self, path: &str) -> VfsResult<Box<dyn VMetadata>>;
    async fn mkdir(&self, path: &str) -> VfsResult<()>;
    async fn mv(&self, from: &str, to: &str) -> VfsResult<()>;
    async fn open(&self, path: &str, options: OpenOptions) -> VfsResult<Box<dyn VFile>>;
    async fn read_dir(
        &self,
        path: &str,
        skip_token: Option<String>,
    ) -> VfsResult<(Vec<Box<dyn VMetadata>>, Option<String>)>;
    async fn rm(&self, path: &str) -> VfsResult<()>;
}
