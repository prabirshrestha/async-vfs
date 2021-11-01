cfg_if::cfg_if! {
    if #[cfg(feature = "runtime-smol")] {
        pub(crate) use std::path::{Path, PathBuf};
    } else if #[cfg(feature = "runtime-tokio")] {
        pub(crate) use std::path::{Path, PathBuf};
    } else if #[cfg(feature = "runtime-async-std")] {
        pub(crate) use async_std::path::{Path, PathBuf};
    } else {
        compile_error!("async-vfs:
You must enable one of the three runtime feature flags
to use this crate: async-std/smol/tokio."
);
    }
}
