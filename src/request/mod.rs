pub mod authed;
mod body;
mod header;

pub use header::*;

use crate::error::{AppError, HttpError, SessionErrors};
use crate::request::body::parse_body;
pub(crate) use crate::request::header::Header;
use crate::route::handle_route;
use crate::session::{AppState, Session};
use async_std::io::{ReadExt, WriteExt};
use async_std::net::TcpStream;
use async_std::prelude::StreamExt;
use async_std::{net, task};
use std::net::Shutdown;

pub struct HttpRequest {
    pub header: Header,
    pub body: Option<String>,
}

impl HttpRequest {
    fn new(req: &str) -> Result<HttpRequest, HttpError> {
        let header = Header::new(req)?;
        let body = parse_body(req);

        Ok(Self { header, body })
    }

    pub fn get_body(&self) -> Result<&str, HttpError> {
        self.body
            .as_deref()
            .ok_or_else(|| HttpError::UnexpectedRequest("body not found".into()))
    }

    pub fn require_session(&self, state: &AppState) -> Result<Session, AppError> {
        let id = self
            .header
            .session_cookie
            .as_ref()
            .ok_or(SessionErrors::InvalidSession)?;

        state
            .get_session(id)?
            .ok_or(SessionErrors::InvalidSession.into())
    }

    pub fn optional_session(&self, state: &AppState) -> Result<Option<Session>, AppError> {
        if let Some(id) = self.header.get_session_cookie() {
            match state.get_session(id)? {
                Some(session) => Ok(Some(session)),
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

pub async fn run() -> Result<(), AppError> {
    let app_state = AppState::new()?;
    let listener = net::TcpListener::bind("[::]:8080").await?;
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let state = app_state.clone();
        let mut stream = stream?;
        task::spawn(async move {
            if let Err(e) = handle_connection(&mut stream, &state).await {
                eprintln!("{:?}", e);
            }
        });
    }
    Ok(())
}

async fn handle_connection(stream: &mut TcpStream, state: &AppState) -> Result<(), AppError> {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let request = String::from_utf8_lossy(&buffer[..n]);
    let http_request = HttpRequest::new(&request)?;
    handle_route(&http_request, stream, state).await?;
    stream.flush().await?;
    stream.shutdown(Shutdown::Write)?;
    Ok(())
}
