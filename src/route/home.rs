use crate::error::HttpError;
use crate::http::get_response;
use async_std::io::WriteExt;
use async_std::net::TcpStream;
use std::fs;

pub async fn home(stream: &mut TcpStream) -> Result<(), HttpError> {
    let html_template = fs::read_to_string("html/index.html")?;
    let mut response = get_response(&html_template);
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
