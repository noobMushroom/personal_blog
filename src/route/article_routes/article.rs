use crate::articles::article::Article;
use crate::error::AppError;
use crate::http::get_response;
use crate::request::HttpRequest;
use crate::route::article_routes::get_article_or_404;
use crate::session::{AppState, Session};
use async_std::io::WriteExt;
use async_std::net::TcpStream;
use tera::Context;

pub async fn article(
    req: &HttpRequest,
    state: &AppState,
    stream: &mut TcpStream,
) -> Result<(), AppError> {
    let optional_session = req.optional_session(&state)?;
    let article = get_article_or_404(&req.header.get_route_uuid(), state, stream).await?;
    let context = get_context(optional_session, &article);
    let render = state.tempelates.render("article.html", &context)?;
    let response = get_response(&render);
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

fn get_context(session: Option<Session>, article: &Article) -> Context {
    let mut context = tera::Context::new();
    context.insert("uuid", &article.uuid);
    context.insert("title", &article.title);
    context.insert("content", &article.content);

    match session {
        Some(_) => {
            context.insert("is_admin", &true);
        }
        None => {
            context.insert("is_admin", &false);
        }
    }
    context
}
