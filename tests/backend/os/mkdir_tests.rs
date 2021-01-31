use crate::testutils::data_dir;
use async_vfs::backend::OsFs;
use async_vfs::*;

#[async_std::test]
async fn mkdir_ok() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    let path = "/mkdir1";
    assert_eq!(vfs.exists(path).await?, false);
    vfs.mkdir(path).await?;
    assert_eq!(vfs.exists(path).await?, true);
    let metadata = vfs.metadata(path).await?;
    assert_eq!(metadata.is_dir(), true);
    assert_eq!(metadata.is_file(), false);
    assert_eq!(metadata.len(), 0);
    assert_eq!(metadata.path(), "/mkdir1");
    vfs.rm(path).await?;

    let path = "/dir2/mkdir2";
    assert_eq!(vfs.exists(path).await?, false);
    vfs.mkdir(path).await?;
    assert_eq!(vfs.exists(path).await?, true);
    let metadata = vfs.metadata(path).await?;
    assert_eq!(metadata.is_dir(), true);
    assert_eq!(metadata.is_file(), false);
    assert_eq!(metadata.len(), 0);
    assert_eq!(metadata.path(), "/dir2/mkdir2");
    vfs.rm(path).await?;
    Ok(())
}

#[async_std::test]
async fn mkdir_fail_when_using_path_without_forward_slash_prefix() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    match vfs.mkdir("mkdir1").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs.mkdir("dir1/mkdir2").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    Ok(())
}

#[async_std::test]
async fn mkdir_fail_when_include_dotdot() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    match vfs.mkdir("../mkdirdot1").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs.mkdir("/../mkdirdot2").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs.mkdir("/dir1/../mkdirdot3").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    Ok(())
}
