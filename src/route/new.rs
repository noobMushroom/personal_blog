use crate::articles::article::Article;
use crate::articles::articles::{ArticleIndex, ArticleMeta};
use crate::error::AppError;
use crate::http::get_response;
use crate::http::get_successful_article_add;
use crate::request::authed::AuthedRequest;
use crate::request::HttpRequest;
use crate::session::AppState;
use async_std::io::WriteExt;
use async_std::net::TcpStream;
use async_std::path::Path;

pub async fn get_article_html(
    state: &AppState,
    req: &HttpRequest,
    stream: &mut TcpStream,
) -> Result<(), AppError> {
    let context = tera::Context::new();
    let render = state.tempelates.render("add_article.html", &context)?;
    let response = get_response(&render);
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

pub async fn add_article(
    req: &HttpRequest,
    state: &AppState,
    stream: &mut TcpStream,
) -> Result<(), AppError> {
    let session = AuthedRequest::new(req, state)?;
    let path = Path::new("Articles").join("index.json");
    let mut article_index = ArticleIndex::read_articles(&path).await?;
    let article = Article::new(req.get_body()?, session.session.user_id)?;
    article.add()?;
    let article_meta = ArticleMeta::new(article.title, article.uuid, article.date);
    article_index.add_articles(article_meta, &path).await?;
    let response = get_successful_article_add();
    stream.write(response.as_bytes()).await?;
    Ok(())
}
