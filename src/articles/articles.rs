use crate::error::HttpError;
use async_std::fs;
use async_std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ArticleMeta {
    pub title: String,
    pub uuid: String,
    pub date: String,
}

pub struct ArticleIndex {
    pub articles: Vec<ArticleMeta>,
}

impl ArticleIndex {
    pub async fn read_articles(path: &Path) -> Result<Self, HttpError> {
        let articles_meta = fs::read_to_string(&path).await?;
        let articles: Vec<ArticleMeta> = serde_json::from_str(&articles_meta)?;
        Ok(Self { articles })
    }
}
