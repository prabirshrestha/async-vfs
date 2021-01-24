use async_vfs::backend::OsFs;
use async_vfs::*;

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[async_std::test]
async fn exists_true_for_existing_file() -> VfsResult<()> {
    let vfs = OsFs::new(CARGO_MANIFEST_DIR);

    assert_eq!(vfs.exists("/Cargo.toml").await?, true);
    assert_eq!(vfs.exists("/src/lib.rs").await?, true);

    Ok(())
}

#[async_std::test]
async fn exists_true_for_existing_dir() -> VfsResult<()> {
    let vfs = OsFs::new(CARGO_MANIFEST_DIR);

    assert_eq!(vfs.exists("/src").await?, true);
    assert_eq!(vfs.exists("/src/backend").await?, true);

    Ok(())
}

#[async_std::test]
async fn exists_false_for_non_existent_file() -> VfsResult<()> {
    let vfs = OsFs::new(CARGO_MANIFEST_DIR);

    assert_eq!(vfs.exists("/nonexistent.file").await?, false);
    assert_eq!(vfs.exists("/src/nonexistent.file").await?, false);

    Ok(())
}

#[async_std::test]
async fn exists_fail_when_using_path_without_forward_slash_prefix() -> VfsResult<()> {
    let vfs = OsFs::new(CARGO_MANIFEST_DIR);

    match vfs.exists("Cargo.toml").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw VfsError::InvalidAbsolutePath"),
    }

    match vfs.exists("nonexistent.file").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw VfsError::InvalidAbsolutePath"),
    }

    Ok(())
}

#[async_std::test]
async fn exists_fail_when_include_dotdot() -> VfsResult<()> {
    let vfs = OsFs::new(CARGO_MANIFEST_DIR);

    match vfs.exists("/../nonexistent.file").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw VfsError::InvalidAbsolutePath"),
    }

    match vfs.exists("/src/../nonexistent.file").await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw VfsError::InvalidAbsolutePath"),
    }

    Ok(())
}
