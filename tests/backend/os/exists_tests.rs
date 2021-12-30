use crate::testutils::{async_test, data_dir};
use async_vfs::backend::OsFs;
use async_vfs::*;

#[async_test]
async fn exists_true_for_existing_file() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    assert_eq!(vfs.exists("/file1a.txt").await?, true);
    assert_eq!(vfs.exists("/dir1/filed1a.txt").await?, true);

    Ok(())
}

#[async_test]
async fn exists_true_for_existing_dir() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    assert_eq!(vfs.exists("/dir1").await?, true);
    assert_eq!(vfs.exists("/dir2/dir3").await?, true);

    Ok(())
}

#[async_test]
async fn exists_false_for_non_existent_file() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    assert_eq!(vfs.exists("/nonexistent.file").await?, false);
    assert_eq!(vfs.exists("/dir1/nonexistent.file").await?, false);

    Ok(())
}

#[async_test]
async fn exists_fail_when_using_path_without_forward_slash_prefix() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    match vfs.exists("file1a.txt").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw VfsError::InvalidAbsolutePath"),
    }

    match vfs.exists("nonexistent.file").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw VfsError::InvalidAbsolutePath"),
    }

    Ok(())
}

#[async_test]
async fn exists_fail_when_include_dotdot() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    match vfs.exists("/../nonexistent.file").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw VfsError::InvalidAbsolutePath"),
    }

    match vfs.exists("/dir1/../nonexistent.file").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw VfsError::InvalidAbsolutePath"),
    }

    Ok(())
}
