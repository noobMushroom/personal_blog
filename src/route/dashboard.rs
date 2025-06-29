use crate::error::AppError;
use crate::html::{get_html, get_html_template};
use crate::http::get_response;
use crate::session::AppState;
use async_std::io::WriteExt;
use async_std::net::TcpStream;

pub async fn dashboard(stream: &mut TcpStream, state: &AppState) -> Result<(), AppError> {
    let html_template = get_html_template("dashboard.html").await?;
    let response = get_response(&html_template);
    let response = get_html(response, "{{username}}", "test");
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
