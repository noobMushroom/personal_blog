use crate::articles;
use crate::error::AppError;
use crate::http::get_response;
use crate::request::HttpRequest;
use crate::session::AppState;
use async_std::io::WriteExt;
use async_std::net::TcpStream;

pub async fn home(
    stream: &mut TcpStream,
    state: &AppState,
    req: &HttpRequest,
) -> Result<(), AppError> {
    let session = req.optional_session(state)?;
    let mut context = tera::Context::new();
    let articles_meta = articles::articles::ArticleIndex::read_articles(
        async_std::path::Path::new("Articles/index.json"),
    )
    .await?;
    match session {
        Some(session) => {
            context.insert("articles", &articles_meta.articles);
            context.insert("is_admin", &true);
        }
        None => {
            context.insert("articles", &articles_meta.articles);
            context.insert("is_admin", &false);
        }
    }
    let render = state.tempelates.render("index.html", &context)?;
    let response = get_response(&render);
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
