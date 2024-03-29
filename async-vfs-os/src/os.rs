use crate::fs_shims::{fs, read_dir, Path, PathBuf};
use async_vfs::{async_trait, OpenOptions, VFile, VMetadata, Vfs, VfsError, VfsResult};
use futures_lite::StreamExt;
use std::{pin::Pin, time::SystemTime};

pub struct OsFs {
    root: PathBuf,
}

struct VOsMetadata {
    path: String,
    is_file: bool,
    len: u64,
    mtime: u64,
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
    fn get_real_path(&self, path: &str) -> VfsResult<PathBuf> {
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

    fn mtime(&self) -> u64 {
        self.mtime
    }
}

#[async_trait]
impl Vfs for OsFs {
    async fn ls(
        &self,
        path: &str,
        _skip_token: Option<String>,
    ) -> VfsResult<(Vec<Box<dyn VMetadata>>, Option<String>)> {
        let mut dir = read_dir(self.get_real_path(path)?).await?;

        let mut list: Vec<Box<dyn VMetadata>> = Vec::new();
        while let Some(entry) = dir.next().await {
            let entry = entry?;
            let metadata = entry.metadata().await?;
            let path = entry.path();
            let mtime = match metadata.modified() {
                Ok(time) => to_timestamp(&time),
                _ => 0,
            };

            let vmetadata = if metadata.is_dir() {
                VOsMetadata {
                    is_file: false,
                    len: 0,
                    path: self.get_vfs_path(&path)?,
                    mtime,
                }
            } else {
                VOsMetadata {
                    is_file: true,
                    len: metadata.len(),
                    path: self.get_vfs_path(&path)?,
                    mtime,
                }
            };

            list.push(Box::new(vmetadata));
        }
        Ok((list, None))
    }

    async fn cp(&self, from: &str, to: &str) -> VfsResult<()> {
        fs::copy(self.get_real_path(from)?, self.get_real_path(to)?).await?;
        Ok(())
    }

    async fn metadata(&self, path: &str) -> VfsResult<Box<dyn VMetadata>> {
        let path = self.get_real_path(path)?;

        let metadata = fs::metadata(&path).await?;
        let mtime = match metadata.modified() {
            Ok(time) => to_timestamp(&time),
            _ => 0,
        };

        let vmetadata = if metadata.is_dir() {
            VOsMetadata {
                is_file: false,
                len: 0,
                path: self.get_vfs_path(&path)?,
                mtime,
            }
        } else {
            VOsMetadata {
                is_file: true,
                len: metadata.len(),
                path: self.get_vfs_path(&path)?,
                mtime,
            }
        };
        Ok(Box::new(vmetadata))
    }

    async fn mkdir(&self, path: &str) -> VfsResult<()> {
        Ok(fs::create_dir(self.get_real_path(path)?).await?)
    }

    async fn mkdir_all(&self, path: &str) -> VfsResult<()> {
        Ok(fs::create_dir_all(self.get_real_path(path)?).await?)
    }

    async fn mv(&self, from: &str, to: &str) -> VfsResult<()> {
        Ok(fs::rename(self.get_real_path(from)?, self.get_real_path(to)?).await?)
    }

    async fn open(
        &self,
        path: &str,
        options: OpenOptions,
    ) -> VfsResult<Pin<Box<dyn VFile + Send>>> {
        let raw_path = self.get_real_path(path)?;

        let file = fs::OpenOptions::new()
            .read(options.has_read())
            .write(options.has_write())
            .create(options.has_create())
            .append(options.has_append())
            .truncate(options.has_truncate())
            .open(raw_path)
            .await?;

        #[cfg(all(
            feature = "runtime-tokio",
            not(feature = "runtime-smol"),
            not(feature = "runtime-async-std")
        ))]
        let file = async_compat::Compat::new(file);

        Ok(Pin::from(Box::new(file)))
    }

    async fn rm(&self, path: &str) -> VfsResult<()> {
        let path = self.get_real_path(path)?;

        let metadata = fs::metadata(&path).await?;

        if metadata.is_dir() {
            Ok(fs::remove_dir(path).await?)
        } else {
            Ok(fs::remove_file(path).await?)
        }
    }
}

fn to_timestamp(time: &SystemTime) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
