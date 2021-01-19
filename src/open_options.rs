#[derive(Debug, Default)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    create: bool,
    append: bool,
    truncate: bool,
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        Default::default()
    }

    pub fn read(mut self, read: bool) -> OpenOptions {
        self.read = read;
        self
    }

    pub fn write(mut self, write: bool) -> OpenOptions {
        self.write = write;
        self
    }

    pub fn create(mut self, create: bool) -> OpenOptions {
        self.create = create;
        self
    }

    pub fn append(mut self, append: bool) -> OpenOptions {
        self.append = append;
        self
    }

    pub fn truncate(mut self, truncate: bool) -> OpenOptions {
        self.truncate = truncate;
        self
    }
}
