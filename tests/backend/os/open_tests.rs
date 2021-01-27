use crate::testutils::data_dir;
use async_std::prelude::*;
use async_vfs::backend::OsFs;
use async_vfs::*;

#[async_std::test]
async fn open_read_only() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    let mut buf = String::new();
    let mut file = vfs
        .open("/file1a.txt", OpenOptions::new().read(true))
        .await?;
    file.read_to_string(&mut buf).await?;
    assert_eq!(buf.as_str(), "contents for file 1 a in /\n");

    let mut buf = String::new();
    let mut file = vfs
        .open("/dir1/empty.txt", OpenOptions::new().read(true))
        .await?;
    file.read_to_string(&mut buf).await?;
    assert!(true, buf.is_empty());

    let mut buf = String::new();
    let mut file = vfs
        .open("/dir1/filed1a.txt", OpenOptions::new().read(true))
        .await?;
    file.read_to_string(&mut buf).await?;
    assert_eq!(buf.as_str(), "filed1a.txt here\n");

    Ok(())
}
