# async-vfs-os

Async virtual filesystem for [async-vfs](https://crates.io/crates/async-vfs) for OS filesystem backend.

Support async-std/smol/tokio runtimes.

## Usage

Adding dependencies in `Cargo.toml`. Use one of the following runtimes.

```
async-vfs = "x.x.x"
async-vfs-os = { version = "x.x.x", features = ["runtime-async-std"] } # for async-std runtime
async-vfs-os = { version = "x.x.x", features = ["runtime-smol"] } # for smol runtime
async-vfs-os = { version = "x.x.x", features = ["runtime-tokio"] } # for tokio runtime
```

Code:
```
use async_vfs::Vfs;
use async_vfs_os::OsFs;

Vfs vfs = OsFs::new("/tmp");
```
