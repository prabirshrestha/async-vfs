use async_vfs::backend::OsFs;
use async_vfs::*;

const CARGO_MANFIFST_DIR: &'static str = env!("CARGO_MANIFEST_DIR");

fn data_dir() -> String {
    return CARGO_MANFIFST_DIR.to_owned() + "/tests/data";
}

#[async_std::test]
async fn metadata_ok_for_file() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    let metadata = vfs.metadata("/file1a.txt").await?;
    assert_eq!(metadata.is_file(), true);
    assert_eq!(metadata.is_dir(), false);
    assert_eq!(metadata.len(), 27);
    assert_eq!(metadata.path(), "/file1a.txt");

    let metadata = vfs.metadata("/dir1/empty.txt").await?;
    assert_eq!(metadata.is_file(), true);
    assert_eq!(metadata.is_dir(), false);
    assert_eq!(metadata.len(), 0);
    assert_eq!(metadata.path(), "/dir1/empty.txt");

    let metadata = vfs.metadata("/dir1/filed1a.txt").await?;
    assert_eq!(metadata.is_file(), true);
    assert_eq!(metadata.is_dir(), false);
    assert_eq!(metadata.len(), 17);
    assert_eq!(metadata.path(), "/dir1/filed1a.txt");

    Ok(())
}

#[async_std::test]
async fn metadata_ok_for_dir() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    let metadata = vfs.metadata("/dir1").await?;
    assert_eq!(metadata.is_file(), false);
    assert_eq!(metadata.is_dir(), true);
    assert_eq!(metadata.len(), 0);
    assert_eq!(metadata.path(), "/dir1");

    let metadata = vfs.metadata("/dir2/dir3").await?;
    assert_eq!(metadata.is_file(), false);
    assert_eq!(metadata.is_dir(), true);
    assert_eq!(metadata.len(), 0);
    assert_eq!(metadata.path(), "/dir2/dir3");

    Ok(())
}

#[async_std::test]
async fn metadata_fail_for_non_existent_file() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    match vfs.metadata("/nonexistent.file").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs.metadata("/dir1/nonexistent.file").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    Ok(())
}
