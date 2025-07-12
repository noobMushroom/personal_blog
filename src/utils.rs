use async_std::path::{Path, PathBuf};
use url_encor::Encoder;

pub fn extract_from_string(body: &str, key: &str) -> Option<String> {
    body.split('&').find_map(|value| {
        let mut parts = value.splitn(2, '=');
        match (parts.next(), parts.next()) {
            (Some(k), Some(v)) if k == key => Some(v.to_string().url_decode()),
            _ => None,
        }
    })
}
///Returns the path of the index.json file for all the articles
pub fn get_articles_index_path() -> PathBuf {
    Path::new("Articles").join("index.json")
}
