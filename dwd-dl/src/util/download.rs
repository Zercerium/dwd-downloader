use std::borrow::Cow;

use super::file::File;

pub fn download_text(url: &str, client: Option<&ureq::Agent>) -> String {
    let client = client
        .map(Cow::Borrowed)
        .unwrap_or_else(|| Cow::Owned(create_client()));

    client.get(url).call().unwrap().into_string().unwrap()
}

pub fn download_body(url: &str, client: Option<&ureq::Agent>) -> bytes::Bytes {
    let client = client
        .map(Cow::Borrowed)
        .unwrap_or_else(|| Cow::Owned(create_client()));
    let mut response_reader = client.get(url).call().unwrap().into_reader();
    let mut body = Vec::new();
    let _ = response_reader.read_to_end(&mut body);
    bytes::Bytes::from(body)
}

pub fn download_file(url: &str, client: Option<&ureq::Agent>) -> File {
    let body = download_body(url, client);
    File::new(url.rsplit('/').next().unwrap().to_string(), body)
}

pub fn create_client() -> ureq::Agent {
    ureq::AgentBuilder::new().build()
}
