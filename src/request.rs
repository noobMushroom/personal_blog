use crate::error::HttpError;
use crate::http::get_route;
use crate::route::handle_route;
use async_std::io::{ReadExt, WriteExt};
use async_std::net::TcpStream;
use async_std::prelude::StreamExt;
use async_std::{net, task};
use std::net::Shutdown;

pub async fn run() -> std::io::Result<()> {
    let listener = net::TcpListener::bind("[::]:8080").await?;
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let mut stream = stream?;
        task::spawn(async move {
            if let Err(e) = handle_connction(&mut stream).await {
                eprintln!("{:?}", e);
            }
        });
    }
    Ok(())
}

async fn handle_connction(stream: &mut TcpStream) -> Result<(), HttpError> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await?;
    let request = String::from_utf8_lossy(&buffer[..]);
    let header = get_route(&request)?;
    handle_route(&header, stream).await;
    stream.flush().await?;
    stream.shutdown(Shutdown::Write)?;
    Ok(())
}
