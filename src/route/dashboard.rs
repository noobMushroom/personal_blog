use crate::error::{AppError, SessionErrors};
use crate::html::{get_html, get_html_template};
use crate::http::get_response;
use crate::request::HttpRequest;
use crate::session::AppState;
use async_std::io::WriteExt;
use async_std::net::TcpStream;

pub async fn dashboard(
    stream: &mut TcpStream,
    state: &AppState,
    req: &HttpRequest,
) -> Result<(), AppError> {
    let html_template = get_html_template("dashboard.html").await?;
    let response = get_response(&html_template);
    let session = state
        .get_session(req.header.get_session_cookie()?)?
        .ok_or_else(|| SessionErrors::InvalidSession)?;
    let response = get_html(response, "{{username}}", &session.username);
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
