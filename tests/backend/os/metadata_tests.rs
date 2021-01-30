use crate::testutils::data_dir;
use async_vfs::backend::OsFs;
use async_vfs::*;

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

#[async_std::test]
async fn metadata_fail_when_using_path_without_forward_slash_prefix() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    match vfs.metadata("file1a.txt").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs.metadata("dir1/filed1a.txt").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    Ok(())
}

#[async_std::test]
async fn metadata_fail_when_include_dotdot() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    match vfs.metadata("../mod.rs").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs.metadata("/../file1a.txt").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs.metadata("/dir1/../file1a.txt").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    Ok(())
}
