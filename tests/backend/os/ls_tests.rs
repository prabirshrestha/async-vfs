use crate::testutils::{async_test, data_dir};
use async_vfs::backend::OsFs;
use async_vfs::*;

#[async_test]
#[ignore]
async fn ls_root() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    let (entries, skip_token) = vfs.ls("/", None).await?;
    assert_eq!(skip_token, None);
    assert_eq!(entries.len(), 4);

    for entry in entries {
        match entry.path() {
            "/dir1" => {
                assert_eq!(entry.is_dir(), true);
                assert_eq!(entry.is_file(), false);
                assert_eq!(entry.len(), 0);
            }
            "/dir2" => {
                assert_eq!(entry.is_dir(), true);
                assert_eq!(entry.is_file(), false);
                assert_eq!(entry.len(), 0);
            }
            "/file1a.txt" => {
                assert_eq!(entry.is_dir(), false);
                assert_eq!(entry.is_file(), true);
                assert_eq!(entry.len(), 27);
            }
            "/file1b.txt" => {
                assert_eq!(entry.is_dir(), false);
                assert_eq!(entry.is_file(), true);
                assert_eq!(entry.len(), 0);
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

#[async_test]
#[ignore]
async fn ls_non_root() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    let (entries, skip_token) = vfs.ls("/dir2", None).await?;
    assert_eq!(skip_token, None);
    assert_eq!(entries.len(), 2);

    for entry in entries {
        match entry.path() {
            "/dir2/dir3" => {
                assert_eq!(entry.is_dir(), true);
                assert_eq!(entry.is_file(), false);
                assert_eq!(entry.len(), 0);
            }
            "/dir2/filed2a.txt" => {
                assert_eq!(entry.is_dir(), false);
                assert_eq!(entry.is_file(), true);
                assert_eq!(entry.len(), 0);
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

#[async_test]
async fn ls_fail_when_using_path_without_forward_slash_prefix() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    match vfs.ls("dir1", None).await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs.ls("dir2/dir3", None).await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    Ok(())
}

#[async_test]
async fn ls_fail_when_include_dotdot() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    match vfs.ls("../data", None).await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs.rm("/../data").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs.rm("/dir2/../dir3").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    Ok(())
}
