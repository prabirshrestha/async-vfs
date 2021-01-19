use async_trait::async_trait;

use crate::VfsResult;

#[async_trait]
pub trait Vfs {
    fn path_separator(&self) -> char;

    async fn create_dir(&self, path: &str) -> VfsResult<()>;
    async fn remove_dir(&self, path: &str) -> VfsResult<()>;
}
