use async_trait::async_trait;

use crate::VfsResult;

pub struct VfsMetadata {}

#[async_trait]
pub trait Vfs {
    fn path_separator(&self) -> char;

    async fn create_dir(&self, path: &str) -> VfsResult<()>;
    async fn remove_dir(&self, path: &str) -> VfsResult<()>;

    async fn remove_file(&self, path: &str) -> VfsResult<()>;
    async fn rename(&self, from: &str, to: &str) -> VfsResult<()>;

    async fn metadata(&self, path: &str) -> VfsResult<VfsMetadata>;
}
