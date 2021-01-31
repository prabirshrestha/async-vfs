use crate::testutils::data_dir;
use async_vfs::backend::OsFs;
use async_vfs::*;

#[async_std::test]
async fn rm_ok_for_file() -> VfsResult<()> {
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
async fn rm_ok_for_empty_dir() -> VfsResult<()> {
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

#[async_std::test]
async fn rm_fail_for_non_existent_file() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    let path = "/rm_non_existent.file";
    assert_eq!(vfs.exists(path).await?, false);
    match vfs.rm(path).await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    let path = "/dir1/rm_non_existent.file";
    assert_eq!(vfs.exists(path).await?, false);
    match vfs.rm(path).await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    Ok(())
}

#[async_std::test]
async fn rm_fail_when_using_path_without_forward_slash_prefix() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    match vfs.rm("file1a.txt").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs.rm("dir1/filed1a.txt").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    Ok(())
}

#[async_std::test]
async fn rm_fail_when_include_dotdot() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    match vfs.rm("../mod.rs").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs.rm("/../file1a.txt").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs.rm("/dir1/../file1a.txt").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    Ok(())
}
