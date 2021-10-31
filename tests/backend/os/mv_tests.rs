/*
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

#[async_std::test]
async fn mv_dir() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());
    assert_eq!(vfs.exists("/dir1/dir2").await?, false);
    vfs.mv("/dir2", "/dir1/dir2").await?;
    assert_eq!(vfs.exists("/dir1/dir2").await?, true);
    vfs.mv("/dir1/dir2", "/dir2").await?;
    Ok(())
}

#[async_std::test]
async fn mv_fail_when_using_path_without_forward_slash_prefix() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    match vfs.mv("file1a.txt", "file1amv.txt").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs.mv("dir1/filed1a.txt", "dir1/filed1amv.txt").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    Ok(())
}

#[async_std::test]
async fn mv_fail_when_include_dotdot() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    match vfs.mv("../mod.rs", "../mod2.rs").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs.mv("/../file1a.txt", "/../file1amv.txt").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs.mv("/dir1/../file1a.txt", "/dir1/../file1amv.txt").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    Ok(())
}
*/
