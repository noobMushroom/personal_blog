use crate::error::AppError;
use crate::html::get_html_template;
use crate::http::{get_failed_login_with_body, get_response, get_successful_login};
use crate::session::Session;
use async_std::io::WriteExt;
use async_std::net::TcpStream;

pub async fn login(stream: &mut TcpStream) -> Result<(), AppError> {
    let html_template = get_html_template("login.html").await?;
    let response = get_response(&html_template);
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

pub async fn login_with_body(
    stream: &mut TcpStream,
    session: Result<Session, AppError>,
) -> Result<(), AppError> {
    match session {
        Ok(user) => {
            let html = get_successful_login(&user.uuid);
            stream.write_all(html.as_bytes()).await?;
        }
        Err(e) => {
            let html = get_failed_login_with_body().await?;
            eprintln!("error: {}", e);
            stream.write_all(html.as_bytes()).await?;
        }
    }
    Ok(())
}
