use crate::http::get_response;
use async_std::io::WriteExt;
use async_std::net::TcpStream;
use std::fs;

pub async fn login(stream: &mut TcpStream) {
    let html_template = fs::read_to_string("html/login.html").unwrap();
    let mut response = get_response(&html_template);
    stream.write_all(response.as_bytes()).await.unwrap();
}
