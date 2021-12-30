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

    pub fn has_read(&self) -> bool {
        self.read
    }

    pub fn read(mut self, read: bool) -> OpenOptions {
        self.read = read;
        self
    }

    pub fn has_write(&self) -> bool {
        self.write
    }

    pub fn write(mut self, write: bool) -> OpenOptions {
        self.write = write;
        self
    }

    pub fn has_create(&self) -> bool {
        self.create
    }

    pub fn create(mut self, create: bool) -> OpenOptions {
        self.create = create;
        self
    }

    pub fn has_append(&self) -> bool {
        self.append
    }

    pub fn append(mut self, append: bool) -> OpenOptions {
        self.append = append;
        self
    }

    pub fn has_truncate(&self) -> bool {
        self.truncate
    }

    pub fn truncate(mut self, truncate: bool) -> OpenOptions {
        self.truncate = truncate;
        self
    }
}
