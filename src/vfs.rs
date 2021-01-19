use async_trait::async_trait;

#[async_trait]
pub trait Vfs {
    fn path_separator() -> char;
}
