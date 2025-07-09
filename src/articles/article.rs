use crate::error::{AppError, HttpError};
use crate::utils::extract_from_string;
use async_std::path::Path;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufWriter;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    pub uuid: String,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub date: String,
}

impl Article {
    pub fn new(body: &str, user_id: String) -> Result<Self, AppError> {
        let title = extract_from_string(body, "title")
            .ok_or_else(|| HttpError::UnexpectedRequest("No body".into()))?;
        let content = extract_from_string(body, "content")
            .ok_or_else(|| HttpError::UnexpectedRequest("No body".into()))?;
        Ok(Self {
            user_id,
            title,
            content,
            uuid: Uuid::new_v4().to_string(),
            date: chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
        })
    }

    pub fn add(&self) -> Result<(), AppError> {
        let path = Path::new("Articles")
            .join("articles")
            .join(format!("{}.json", self.uuid));
        let writer = BufWriter::new(File::create(path)?);
        serde_json::to_writer_pretty(writer, &self)?;
        Ok(())
    }
}
