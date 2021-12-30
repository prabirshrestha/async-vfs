cfg_if::cfg_if! {
    if #[cfg(feature = "runtime-smol")] {
        pub(crate) use std::path::{Path, PathBuf};
        pub(crate) use smol::fs;
        pub(crate) use smol::fs::read_dir;
    } else if #[cfg(feature = "runtime-tokio")] {
        pub(crate) use std::path::{Path, PathBuf};
        pub(crate) use tokio::fs;
        use tokio_stream::wrappers::ReadDirStream;
        pub async fn read_dir<P: AsRef<Path>>(path: P) -> std::io::Result<ReadDirStream> {
            Ok(ReadDirStream::new(
                fs::read_dir(path).await?
            ))
        }
    } else if #[cfg(feature = "runtime-async-std")] {
        pub(crate) use async_std::path::{Path, PathBuf};
        pub(crate) use async_std::fs;
        pub(crate) use async_std::fs::read_dir;
    } else {
        compile_error!("async-vfs:
You must enable one of the three runtime feature flags
to use this crate: async-std/smol/tokio."
);
    }
}
