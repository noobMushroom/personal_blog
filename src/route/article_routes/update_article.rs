use crate::articles::article::{get_article, Article};
use crate::error::AppError;
use crate::http::get_response;
use crate::request::authed::AuthedRequest;
use async_std::io::WriteExt;
use async_std::net::TcpStream;
use tera::Context;

pub async fn get_update_form(
    authed: AuthedRequest<'_>,
    stream: &mut TcpStream,
) -> Result<(), AppError> {
    let article = get_article(authed.req.header.get_route_uuid())?;
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
    let article = get_article(authed.req.header.get_route_uuid())?;

    Ok(())
}

fn get_context(article: &Article) -> Context {
    let mut context = Context::new();
    context.insert("title", &article.title);
    context.insert("uuid", &article.uuid);
    context.insert("content", &article.content);
    context
}
