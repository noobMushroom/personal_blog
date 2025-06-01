use crate::error::HttpError;
use async_std::fs;
use async_std::path::Path;

pub async fn get_html_template(path: &str) -> Result<String, HttpError> {
    let path = Path::new("html/").join(path);
    fs::read_to_string(&path)
        .await
        .map_err(|e| HttpError::IoWithContext {
            source: e,
            path: path.display().to_string(),
        })
}

pub fn get_html(html: String, replace_from: &str, replace_to: &str) -> String {
    html.replace(replace_from, replace_to)
}
