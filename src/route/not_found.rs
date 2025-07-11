use crate::error::AppError;
use crate::http::get_page_not_found;
use crate::request::HttpRequest;
use crate::session::AppState;
use async_std::io::WriteExt;
use async_std::net::TcpStream;

pub async fn page_not_found(
    req: &HttpRequest,
    state: &AppState,
    stream: &mut TcpStream,
    link: &str,
) -> Result<(), AppError> {
    let mut context = tera::Context::new();
    context.insert("link", link);
    let render = state.tempelates.render("page_not_found.html", &context)?;
    let response = get_page_not_found(&render);
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
