use crate::articles::articles::ArticleIndex;
use crate::error::AppError;
use crate::http::get_response;
use crate::request::HttpRequest;
use crate::session::{AppState, Session};
use async_std::io::WriteExt;
use async_std::net::TcpStream;
use tera::Context;

pub async fn home(
    stream: &mut TcpStream,
    state: &AppState,
    req: &HttpRequest,
) -> Result<(), AppError> {
    let session = req.optional_session(state)?;
    let articles_meta =
        ArticleIndex::read_articles(async_std::path::Path::new("Articles/index.json")).await?;
    let context = generate_context(&articles_meta, &session);
    let render = state.tempelates.render("index.html", &context)?;
    let response = get_response(&render);
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

fn generate_context(articles_meta: &ArticleIndex, session: &Option<Session>) -> Context {
    let mut context = tera::Context::new();
    match session {
        Some(session) => {
            context.insert("is_admin", &true);
            context.insert("username", &session.username);
        }
        None => context.insert("is_admin", &false),
    }
    context.insert("articles", &articles_meta.articles);
    context
}

fn generate_guest_context(articles_meta: &ArticleIndex) -> Context {
    let mut context = tera::Context::new();
    context.insert("articles", &articles_meta.articles);
    context.insert("is_admin", &false);
    context
}
