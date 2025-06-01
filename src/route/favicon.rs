use crate::error::HttpError;
use async_std::io::WriteExt;
use async_std::net::TcpStream;

pub async fn favicon(stream: &mut TcpStream) -> Result<(), HttpError> {
    let response = "HTTP/1.1 204 No Content\r\n\r\n";
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
