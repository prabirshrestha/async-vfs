use crate::{OpenOptions, VFile, VMetadata, Vfs, VfsError, VfsResult};
use async_std::{fs, path::PathBuf};
use async_std::{path::Path, prelude::*};
use async_trait::async_trait;
use std::pin::Pin;

pub struct OsFs {
    root: PathBuf,
}

struct VOsMetadata {
    path: String,
    is_file: bool,
    len: u64,
}

impl OsFs {
    pub fn new(root: &str) -> Self {
        OsFs {
            root: PathBuf::from(root),
        }
    }

    // if root   => "/home"
    //    path   => "/docs"
    //    result => "/home/docs"
    fn get_raw_path(&self, path: &str) -> VfsResult<PathBuf> {
        if path.contains("..") {
            return Err(VfsError::InvalidAbsolutePath {
                path: path.to_owned(),
            });
        }

        if path.starts_with("/") {
            Ok(self.root.join(&path[1..]))
        } else {
            Err(VfsError::InvalidAbsolutePath {
                path: String::from(path),
            })
        }
    }

    // if root => "/home"
    //    path => /home/doc
    //    result => /doc
    fn get_vfs_path(&self, path: &Path) -> VfsResult<String> {
        let pathstr = path.to_str().ok_or(VfsError::InvalidAbsolutePath {
            path: String::from(""),
        })?;

        if pathstr.contains("..") {
            return Err(VfsError::InvalidAbsolutePath {
                path: pathstr.to_owned(),
            });
        }

        if !path.is_absolute() {
            return Err(VfsError::InvalidAbsolutePath {
                path: pathstr.to_owned(),
            });
        }

        if path.starts_with(&self.root) {
            let res = path
                .strip_prefix(&self.root)
                .or_else(|_| {
                    Err(VfsError::InvalidAbsolutePath {
                        path: pathstr.to_owned(),
                    })
                })?
                .to_str()
                .ok_or_else(|| VfsError::InvalidAbsolutePath {
                    path: pathstr.to_owned(),
                })?;
            Ok("/".to_owned() + res)
        } else {
            Err(VfsError::InvalidAbsolutePath {
                path: pathstr.to_owned(),
            })
        }
    }
}

impl VMetadata for VOsMetadata {
    fn path(&self) -> &str {
        &self.path
    }

    fn is_dir(&self) -> bool {
        !self.is_file
    }

    fn is_file(&self) -> bool {
        self.is_file
    }

    fn len(&self) -> u64 {
        self.len
    }
}

#[async_trait]
impl Vfs for OsFs {
    async fn exists(&self, path: &str) -> VfsResult<bool> {
        Ok(self.get_raw_path(path)?.exists().await)
    }

    async fn ls(
        &self,
        path: &str,
        _skip_token: Option<String>,
    ) -> VfsResult<(Vec<Box<dyn VMetadata>>, Option<String>)> {
        let mut dir = fs::read_dir(self.get_raw_path(path)?).await?;
        let mut list: Vec<Box<dyn VMetadata>> = Vec::new();
        while let Some(entry) = dir.next().await {
            let entry = entry?;
            let metadata = entry.metadata().await?;
            let path = entry.path();

            let vmetadata = if metadata.is_dir() {
                VOsMetadata {
                    is_file: false,
                    len: 0,
                    path: self.get_vfs_path(&path)?,
                }
            } else {
                VOsMetadata {
                    is_file: true,
                    len: metadata.len(),
                    path: self.get_vfs_path(&path)?,
                }
            };

            list.push(Box::new(vmetadata));
        }
        Ok((list, None))
    }

    async fn metadata(&self, path: &str) -> VfsResult<Box<dyn VMetadata>> {
        let path = self.get_raw_path(path)?;
        let metadata = path.metadata().await?;
        let vmetadata = if metadata.is_dir() {
            VOsMetadata {
                is_file: false,
                len: 0,
                path: self.get_vfs_path(&path)?,
            }
        } else {
            VOsMetadata {
                is_file: true,
                len: metadata.len(),
                path: self.get_vfs_path(&path)?,
            }
        };
        Ok(Box::new(vmetadata))
    }

    async fn mkdir(&self, path: &str) -> VfsResult<()> {
        Ok(fs::create_dir(self.get_raw_path(path)?).await?)
    }

    async fn mv(&self, from: &str, to: &str) -> VfsResult<()> {
        Ok(fs::rename(from, to).await?)
    }

    async fn open(&self, path: &str, options: OpenOptions) -> VfsResult<Pin<Box<dyn VFile>>> {
        let raw_path = self.get_raw_path(path)?;
        if raw_path.is_dir().await {
            return Err(VfsError::InvalidFile {
                path: path.to_owned(),
            });
        }
        let file = fs::OpenOptions::new()
            .read(options.has_read())
            .write(options.has_write())
            .create(options.has_create())
            .append(options.has_append())
            .truncate(options.has_truncate())
            .open(raw_path)
            .await?;
        Ok(Pin::from(Box::new(file)))
    }

    async fn rm(&self, path: &str) -> VfsResult<()> {
        let path = self.get_raw_path(path)?;
        let metadata = path.metadata().await?;
        if metadata.is_dir() {
            Ok(fs::remove_dir(path).await?)
        } else {
            Ok(fs::remove_file(path).await?)
        }
    }
}
