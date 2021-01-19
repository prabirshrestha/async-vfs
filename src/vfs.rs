use async_trait::async_trait;

use crate::VfsResult;

pub trait VMetadata {
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
    fn len(&self) -> u64;
}

#[async_trait]
pub trait Vfs {
    async fn create_dir(&self, path: &str) -> VfsResult<()>;
    async fn remove_dir(&self, path: &str) -> VfsResult<()>;

    async fn remove_file(&self, path: &str) -> VfsResult<()>;
    async fn rename(&self, from: &str, to: &str) -> VfsResult<()>;

    async fn metadata(&self, path: &str) -> VfsResult<Box<dyn VMetadata>>;

    async fn exists(&self, path: &str) -> VfsResult<bool>;
}
