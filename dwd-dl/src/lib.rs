use std::sync::OnceLock;

pub mod dwd_source;
pub mod products;
pub mod util;

static DWD_URL: OnceLock<String> = OnceLock::new();

pub fn base_url() -> &'static str {
    DWD_URL.get_or_init(|| {
        std::env::var("DWD_URL").unwrap_or_else(|_| {
            match cfg!(debug_assertions) {
                // We shouldn't stress the DWD Server in Debug builds with testing
                true => "http://localhost/",
                false => "https://opendata.dwd.de/",
            }
            .to_string()
        })
    })
}
