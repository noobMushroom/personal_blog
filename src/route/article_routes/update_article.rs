use crate::articles::article::Article;
use crate::articles::articles::ArticleIndex;
use crate::error::{AppError, HttpError};
use crate::http::{get_response, get_successful_article_add};
use crate::request::authed::AuthedRequest;
use crate::route::article_routes::get_article_or_404;
use crate::utils::extract_from_string;
use async_std::io::WriteExt;
use async_std::net::TcpStream;
use async_std::path::Path;
use tera::Context;

pub async fn get_update_form(
    authed: AuthedRequest<'_>,
    stream: &mut TcpStream,
) -> Result<(), AppError> {
    let article =
        get_article_or_404(&authed.req.header.get_route_uuid(), &authed.state, stream).await?;
    let render = authed
        .state
        .tempelates
        .render("article_update.html", &get_context(&article))?;

    let response = get_response(&render);
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

pub async fn update_article(
    authed: AuthedRequest<'_>,
    stream: &mut TcpStream,
) -> Result<(), AppError> {
    let path = Path::new("Articles").join("index.json");
    let (title, content) = extract_article_body(authed.req.get_body()?)?;
    let mut article =
        get_article_or_404(&authed.req.header.get_route_uuid(), &authed.state, stream).await?;
    article.update(&title, &content)?;
    let mut article_index = ArticleIndex::read_articles(&path).await?;
    article_index
        .update_articles(authed.req.header.get_route_uuid(), &title, &path)
        .await?;
    let response = get_successful_article_add();
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

fn extract_article_body(body: &str) -> Result<(String, String), AppError> {
    let title = extract_from_string(body, "title")
        .ok_or_else(|| HttpError::UnexpectedRequest("No body".into()))?;
    let content = extract_from_string(body, "content")
        .ok_or_else(|| HttpError::UnexpectedRequest("No body".into()))?;
    Ok((title, content))
}

fn get_context(article: &Article) -> Context {
    let mut context = Context::new();
    context.insert("title", &article.title);
    context.insert("uuid", &article.uuid);
    context.insert("content", &article.content);
    context
}
