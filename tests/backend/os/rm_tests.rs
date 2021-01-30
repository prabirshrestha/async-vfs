use crate::testutils::data_dir;
use async_vfs::backend::OsFs;
use async_vfs::*;

#[async_std::test]
async fn rm_file_tests() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    let path = "/rm_empty1.txt";
    assert_eq!(vfs.exists(path).await?, false);
    let _file = vfs
        .open(path, OpenOptions::new().create(true).write(true))
        .await?;
    assert_eq!(vfs.exists(path).await?, true);
    assert_eq!(vfs.metadata(path).await?.is_file(), true);
    vfs.rm(path).await?;
    assert_eq!(vfs.exists(path).await?, false);

    let path = "/dir1/rm_empty2.txt";
    assert_eq!(vfs.exists(path).await?, false);
    let _file = vfs
        .open(path, OpenOptions::new().create(true).write(true))
        .await?;
    assert_eq!(vfs.exists(path).await?, true);
    assert_eq!(vfs.metadata(path).await?.is_file(), true);
    vfs.rm(path).await?;
    assert_eq!(vfs.exists(path).await?, false);

    Ok(())
}

#[async_std::test]
async fn rm_dir_tests() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    let path = "/rm_dir1";
    assert_eq!(vfs.exists(path).await?, false);
    vfs.mkdir(path).await?;
    assert_eq!(vfs.exists(path).await?, true);
    assert_eq!(vfs.metadata(path).await?.is_dir(), true);
    vfs.rm(path).await?;
    assert_eq!(vfs.exists(path).await?, false);

    let path = "/dir1/rm_dir1";
    assert_eq!(vfs.exists(path).await?, false);
    vfs.mkdir(path).await?;
    assert_eq!(vfs.exists(path).await?, true);
    assert_eq!(vfs.metadata(path).await?.is_dir(), true);
    vfs.rm(path).await?;
    assert_eq!(vfs.exists(path).await?, false);

    Ok(())
}
