use std::io::Read;

use bytes::Bytes;
use tar::{Archive, Entry};

pub struct Tarball {
    data: Bytes,
    entries: Vec<TarEntry>,
}

impl Tarball {
    pub fn new(data: Bytes) -> Self {
        let mut archive = Archive::new(data.as_ref());
        let mut entries = Vec::new();
        for entry in archive.entries().unwrap() {
            let entry = entry.unwrap();
            entries.push(TarEntry::from_tar_entry(entry, data.clone()));
        }
        entries.sort_unstable_by(|a, b| a.filename.cmp(&b.filename));
        Self { data, entries }
    }

    pub fn entries(&self) -> &[TarEntry] {
        &self.entries
    }
}

pub struct TarEntry {
    pub filename: String,
    offset: usize,
    size: usize,
    pub data: Bytes,
}

impl TarEntry {
    pub fn new(filename: String, offset: usize, size: usize, data: Bytes) -> Self {
        Self {
            filename,
            offset,
            size,
            data,
        }
    }

    pub fn from_tar_entry<'a, R>(entry: Entry<'a, R>, data: Bytes) -> Self
    where
        R: 'a + Read,
    {
        let name = entry.path().unwrap().display().to_string();
        let offset = entry.raw_file_position() as usize;
        let length = entry.header().size().unwrap() as usize;
        let data = data.slice(offset..offset + length);
        Self::new(name, offset, length, data)
    }
}
