use crate::error::AppError;
use crate::http::{get_failed_login_with_body, get_response, get_successful_login};
use crate::session::{AppState, Session};
use async_std::io::WriteExt;
use async_std::net::TcpStream;

pub async fn login(stream: &mut TcpStream, state: &AppState) -> Result<(), AppError> {
    let context = tera::Context::new();
    let render = state.tempelates.render("login.html", &context)?;
    let response = get_response(&render);
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

pub async fn login_with_body(
    stream: &mut TcpStream,
    state: &AppState,
    session: Result<Session, AppError>,
) -> Result<(), AppError> {
    match session {
        Ok(user) => {
            let html = get_successful_login(&user.uuid);
            stream.write_all(html.as_bytes()).await?;
        }
        Err(e) => {
            let context = tera::Context::new();
            let render = state.tempelates.render("failed_login.html", &context)?;
            let response = get_failed_login_with_body(&render);
            eprintln!("error: {}", e);
            stream.write_all(response.as_bytes()).await?;
        }
    }
    Ok(())
}
