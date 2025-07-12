use crate::error::AppError;
use crate::http::get_logout_response;
use crate::request::authed::AuthedRequest;
use async_std::io::WriteExt;
use async_std::net::TcpStream;

pub async fn logout(authed: AuthedRequest<'_>, stream: &mut TcpStream) -> Result<(), AppError> {
    authed.state.remove_session(&authed.session.uuid)?;
    let response = get_logout_response();
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
