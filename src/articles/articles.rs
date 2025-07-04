use crate::error::AppError;
use async_std::fs;
use async_std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ArticleMeta {
    pub title: String,
    pub uuid: String,
    pub date: String,
}

pub struct ArticleIndex {
    pub articles: Vec<ArticleMeta>,
}

impl ArticleIndex {
    pub async fn read_articles(path: &Path) -> Result<Self, AppError> {
        let articles_meta = fs::read_to_string(&path).await?;
        let articles: Vec<ArticleMeta> = serde_json::from_str(&articles_meta)?;
        Ok(Self { articles })
    }
}
