use crate::error::AppError;
use async_std::fs;
use async_std::path::Path;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufWriter;

#[derive(Deserialize, Serialize, Debug)]
pub struct ArticleMeta {
    pub title: String,
    pub uuid: String,
    pub date: String,
}

impl ArticleMeta {
    pub fn new(title: String, uuid: String, date: String) -> Self {
        Self { title, uuid, date }
    }
}

pub struct ArticleIndex {
    pub articles: Vec<ArticleMeta>,
}

impl ArticleIndex {
    pub fn new() -> Self {
        Self {
            articles: Vec::new(),
        }
    }
    pub async fn read_articles(path: &Path) -> Result<Self, AppError> {
        let metadata = fs::metadata(path).await?;
        if metadata.len() == 0 {
            return Ok(Self::new());
        }
        let articles_meta = fs::read_to_string(&path).await?;
        let articles: Vec<ArticleMeta> = serde_json::from_str(&articles_meta)?;
        Ok(Self { articles })
    }

    pub async fn save_articles(&self, path: &Path) -> Result<(), AppError> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.articles)?;
        Ok(())
    }

    pub async fn add_articles(
        &mut self,
        article_meta: ArticleMeta,
        path: &Path,
    ) -> Result<(), AppError> {
        self.articles.push(article_meta);
        self.save_articles(path).await?;
        Ok(())
    }

    pub async fn update_articles(
        &mut self,
        uuid: &str,
        title: &str,
        path: &Path,
    ) -> Result<(), AppError> {
        match self.articles.iter_mut().find(|meta| meta.uuid == uuid) {
            Some(article) => {
                article.title = title.to_string();
                self.save_articles(path).await?;
            }
            None => {
                return Err(AppError::Custom("Article not found".to_string()));
            }
        };
        Ok(())
    }

    pub async fn delete_articles(&mut self, path: &Path, uuid: &str) -> Result<(), AppError> {
        self.articles.retain(|meta| meta.uuid != uuid);
        self.save_articles(path).await?;
        Ok(())
    }
}
