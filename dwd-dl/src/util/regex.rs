use once_cell::sync::Lazy;
use regex::Regex;
use time::Date;

use super::interval::Interval;

/// Extracts links from a text using a specified regex pattern.
///
/// # Arguments
///
/// * `text` - The input text containing HTML anchor tags.
/// * `regex` - The regular expression pattern used for matching links
///
/// # Returns
///
/// A vector of strings representing the links extracted from the text
///
/// # Examples
///
/// ```
/// use dwd_dl::util::regex::links_in_text;
///
/// let text = r#"
///     <a href="./page1.html">Page 1</a>
///     <a href="./page2.html">Page 2</a>
/// "#;
///
/// let regex_pattern = r#"page\d\.html"#;
/// let result = links_in_text(text, regex_pattern);
///
/// assert_eq!(result, vec!["page1.html", "page2.html"]);
/// ```
///
/// # Panics
///
/// This function will panic if the provided regex pattern is invalid. Make sure the regex is
/// constructed correctly to avoid panics.
pub fn links_in_text(text: &str, regex: &str) -> Vec<String> {
    let mut links = vec![];
    let regex = format!(r#"<a href="(\./)?(?P<url>{})/?">"#, regex);
    let re = regex::Regex::new(&regex).unwrap();
    for cap in re.captures_iter(text) {
        links.push(cap.name("url").unwrap().as_str().to_string());
    }
    links
}

pub fn year_links_in_text(text: &str) -> Vec<i32> {
    let regex = r#"\d{4}"#;
    let years = links_in_text(text, regex);
    years
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

pub fn extract_interval_d8_d8(s: &str) -> Result<Interval<Date>, ()> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<start>\d{8})_(?P<end>\d{8})").unwrap());
    let cap = RE.captures(s).ok_or(())?;
    Ok(Interval::<Date>::parse_str(&cap["start"], &cap["end"]))
}

pub fn extract_d6(s: &str) -> Result<&str, ()> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d{6}").unwrap());
    Ok(RE.find(s).unwrap().as_str())
}

pub fn extract_d8(s: &str) -> Result<&str, ()> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d{8}").unwrap());
    Ok(RE.find(s).unwrap().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_links_in_text() {
        let text = r#"<a href="./test1">test1</a><a href="test2">test2</a>"#;
        let regex = r#"test\d"#;
        let links = links_in_text(text, regex);
        assert_eq!(links, vec!["test1", "test2"]);
    }

    #[test]
    fn year_test_links_in_text() {
        let text = r#"<a href="./2000">2000</a><a href="2001">2001</a>  <a href="2020">2020</a> <a href="./2008/"> gfhjjhg </a>"#;
        let links = year_links_in_text(text);
        assert_eq!(links, vec![2000, 2001, 2020, 2008]);
    }
}
