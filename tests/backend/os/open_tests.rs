use crate::testutils::data_dir;
use async_std::prelude::*;
use async_vfs::backend::OsFs;
use async_vfs::*;

#[async_std::test]
async fn open_read_only() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    let mut buf = String::new();
    let mut file = vfs
        .open("/file1a.txt", OpenOptions::new().read(true))
        .await?;
    file.read_to_string(&mut buf).await?;
    assert_eq!(buf.as_str(), "contents for file 1 a in /\n");

    let mut buf = String::new();
    let mut file = vfs
        .open("/dir1/empty.txt", OpenOptions::new().read(true))
        .await?;
    file.read_to_string(&mut buf).await?;
    assert!(true, buf.is_empty());

    let mut buf = String::new();
    let mut file = vfs
        .open("/dir1/filed1a.txt", OpenOptions::new().read(true))
        .await?;
    file.read_to_string(&mut buf).await?;
    assert_eq!(buf.as_str(), "filed1a.txt here\n");

    Ok(())
}

#[async_std::test]
async fn open_fail_for_dir() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    match vfs.open("/dir1", OpenOptions::new().read(true)).await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw error"),
    }

    match vfs.open("/dir2/dir3", OpenOptions::new().read(true)).await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw error"),
    }

    Ok(())
}

#[async_std::test]
async fn open_create_write_new_empty_file() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    let path = "/open_empty1.txt";
    assert_eq!(vfs.exists(path).await?, false);
    {
        // note: move file to block for explict close via drop
        vfs.open(path, OpenOptions::new().create(true).write(true))
            .await?;
    }
    assert_eq!(vfs.exists(path).await?, true);
    let metadata = vfs.metadata(path).await?;
    assert_eq!(metadata.is_file(), true);
    assert_eq!(metadata.is_dir(), false);
    assert_eq!(metadata.len(), 0);
    assert_eq!(metadata.path(), path);
    vfs.rm(path).await?;
    assert_eq!(vfs.exists(path).await?, false);

    let path = "/dir1/open_empty2.txt";
    assert_eq!(vfs.exists(path).await?, false);
    {
        // note: move file to block for explict close via drop
        vfs.open(path, OpenOptions::new().create(true).write(true))
            .await?;
    }
    assert_eq!(vfs.exists(path).await?, true);
    let metadata = vfs.metadata(path).await?;
    assert_eq!(metadata.is_file(), true);
    assert_eq!(metadata.is_dir(), false);
    assert_eq!(metadata.len(), 0);
    assert_eq!(metadata.path(), path);
    vfs.rm(path).await?;
    assert_eq!(vfs.exists(path).await?, false);

    Ok(())
}

#[async_std::test]
async fn open_create_write_new_file_with_contents() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    let path = "/open_new_file1.txt";
    assert_eq!(vfs.exists(path).await?, false);
    {
        // note: move file to block for explict close via drop
        let mut file = vfs
            .open(path, OpenOptions::new().create(true).write(true))
            .await?;
        file.write(b"Hello \nWorld").await?; // test ASCII
    }
    assert_eq!(vfs.exists(path).await?, true);
    let metadata = vfs.metadata(path).await?;
    assert_eq!(metadata.is_file(), true);
    assert_eq!(metadata.is_dir(), false);
    assert_eq!(metadata.len(), 12);
    assert_eq!(metadata.path(), path);

    {
        // note: move file to block for explict close via drop
        let mut file = vfs.open(path, OpenOptions::new().read(true)).await?;
        let mut buf = vec![0; 1024];
        let len = file.read(&mut buf).await?;
        assert_eq!(len, 12);
        assert_eq!(&buf[0..len], b"Hello \nWorld");
    }

    vfs.rm(path).await?;

    let path = "/dir1/open_new_file2.txt";
    assert_eq!(vfs.exists(path).await?, false);
    {
        // note: move file to block for explict close via drop
        let mut file = vfs
            .open(path, OpenOptions::new().create(true).write(true))
            .await?;
        file.write("Hello \nWorld\nनमस्ते".as_bytes()).await?; // test UTF-8
    }
    assert_eq!(vfs.exists(path).await?, true);
    let metadata = vfs.metadata(path).await?;
    assert_eq!(metadata.is_file(), true);
    assert_eq!(metadata.is_dir(), false);
    assert_eq!(metadata.len(), 31);
    assert_eq!(metadata.path(), path);

    {
        // note: move file to block for explict close via drop
        let mut file = vfs.open(path, OpenOptions::new().read(true)).await?;
        let mut buf = vec![0; 1024];
        let len = file.read(&mut buf).await?;
        assert_eq!(len, 31);
        assert_eq!(&buf[0..len], "Hello \nWorld\nनमस्ते".as_bytes());
    }

    vfs.rm(path).await?;

    Ok(())
}

#[async_std::test]
async fn open_fail_when_using_path_without_forward_slash_prefix() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    match vfs.open("file1a.txt", OpenOptions::new().read(true)).await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs
        .open("dir1/filed1a.txt", OpenOptions::new().read(true))
        .await
    {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    Ok(())
}

#[async_std::test]
async fn open_fail_when_include_dotdot() -> VfsResult<()> {
    let vfs = OsFs::new(&data_dir());

    match vfs.open("../mod.rs", OpenOptions::new().read(true)).await {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs
        .open("/../file1a.txt", OpenOptions::new().read(true))
        .await
    {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    match vfs
        .open("/dir1/../file1a.txt", OpenOptions::new().read(true))
        .await
    {
        Err(_) => assert!(true),
        _ => assert!(false, "should throw Error"),
    }

    Ok(())
}
