use crate::testutils::data_dir;
use async_vfs::backend::OsFs;
use async_vfs::*;

#[async_std::test]
async fn mv_empty_file() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    assert_eq!(vfs.exists("/mv1.txt").await?, false);
    vfs.open("/mv1.txt", OpenOptions::new().create(true).write(true))
        .await?;
    vfs.mv("/mv1.txt", "/mv2.txt").await?;
    assert_eq!(vfs.exists("/mv1.txt").await?, false);
    assert_eq!(vfs.exists("/mv2.txt").await?, true);
    let metadata = vfs.metadata("/mv2.txt").await?;
    assert_eq!(metadata.is_dir(), false);
    assert_eq!(metadata.is_file(), true);
    assert_eq!(metadata.len(), 0);
    vfs.rm("/mv2.txt").await?;

    assert_eq!(vfs.exists("/dir2/mv2.txt").await?, false);
    vfs.open("/dir2/mv2.txt", OpenOptions::new().create(true).write(true))
        .await?;
    vfs.mv("/dir2/mv2.txt", "/dir2/dir3/mvb.ext").await?;
    assert_eq!(vfs.exists("/dir2/mv2.txt").await?, false);
    assert_eq!(vfs.exists("/dir2/dir3/mvb.ext").await?, true);
    let metadata = vfs.metadata("/dir2/dir3/mvb.ext").await?;
    assert_eq!(metadata.is_dir(), false);
    assert_eq!(metadata.is_file(), true);
    assert_eq!(metadata.len(), 0);
    vfs.rm("/dir2/dir3/mvb.ext").await?;

    Ok(())
}
