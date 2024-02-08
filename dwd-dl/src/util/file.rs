use bytes::Bytes;

pub struct File {
    pub name: String,
    pub data: Bytes,
}

impl File {
    pub fn new(name: String, data: Bytes) -> Self {
        Self { name, data }
    }

    pub fn extension(&self) -> Option<&str> {
        self.name.rsplit('.').next()
    }
}
