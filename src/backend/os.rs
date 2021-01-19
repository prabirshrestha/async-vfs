use crate::{OpenOptions, VFile, VMetadata, Vfs, VfsResult};
use async_std::path::PathBuf;
use async_trait::async_trait;

pub struct OsFs {
    root: PathBuf,
}

impl OsFs {
    pub fn new(root: &str) -> Self {
        OsFs {
            root: PathBuf::from(root),
        }
    }

    fn get_path(&self, path: &str) -> PathBuf {
        let p = if path.starts_with('/') {
            &path[1..]
        } else {
            path
        };
        self.root.join(p)
    }
}

#[async_trait]
impl Vfs for OsFs {
    async fn exists(&self, path: &str) -> VfsResult<bool> {
        Ok(self.get_path(path).exists().await)
    }

    async fn metadata(&self, path: &str) -> VfsResult<Box<dyn VMetadata>> {
        todo!()
    }

    async fn mkdir(&self, path: &str) -> VfsResult<()> {
        todo!()
    }

    async fn mv(&self, from: &str, to: &str) -> VfsResult<()> {
        todo!()
    }

    async fn open(&self, path: &str, options: OpenOptions) -> VfsResult<Box<dyn VFile>> {
        todo!()
    }

    async fn read_dir(
        &self,
        path: &str,
        skip_token: Option<String>,
    ) -> VfsResult<(Vec<Box<dyn VMetadata>>, Option<String>)> {
        todo!()
    }

    async fn rm(&self, path: &str) -> VfsResult<()> {
        todo!()
    }
}
