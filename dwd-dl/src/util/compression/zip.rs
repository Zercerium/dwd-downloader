use std::io::Read;

use bytes::Bytes;

struct File {
    name: String,
    id: usize,
}

pub fn extract_file(b: Bytes, search: &str) -> Vec<u8> {
    let cursor = std::io::Cursor::new(b);
    let mut archive = zip::ZipArchive::new(cursor).unwrap();
    let mut archive_indexed = Vec::new();
    for i in 0..archive.len() {
        let file = archive.by_index_raw(i).unwrap();
        archive_indexed.push(File {
            name: file.name().to_string(),
            id: i,
        });
    }
    let id = archive_indexed
        .iter()
        .find(|x| x.name.contains(search))
        .unwrap()
        .id;
    let mut buf = Vec::new();
    let _ = archive.by_index(id).unwrap().read_to_end(&mut buf);
    buf
}
