use crate::articles::article::{get_article, Article};
use crate::error::{AppError, HttpError};
use crate::http::get_page_not_found;
use crate::session::AppState;
use async_std::io::WriteExt;
use async_std::net::TcpStream;
use tera::Context;

pub mod article;
pub mod new;
pub mod update_article;

pub async fn get_article_or_404(
    uuid: &str,
    state: &AppState,
    stream: &mut TcpStream,
) -> Result<Article, AppError> {
    match get_article(uuid) {
        Ok(article) => Ok(article),
        Err(AppError::Http(HttpError::PageNotFound)) => {
            let mut context = Context::new();
            context.insert("link", "Article");
            let render = state.tempelates.render("page_not_found.html", &context)?;
            let response = get_page_not_found(&render);
            stream.write_all(response.as_bytes()).await?;
            Err(AppError::Http(HttpError::PageNotFound)) // Or Ok(()) if you want to swallow it
        }
        Err(e) => Err(e),
    }
}
