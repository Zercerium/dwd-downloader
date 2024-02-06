use std::borrow::Cow;

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

pub fn create_client() -> ureq::Agent {
    ureq::AgentBuilder::new().build()
}
