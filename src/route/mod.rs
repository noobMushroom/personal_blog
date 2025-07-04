use crate::error::{AppError, HttpError};
use crate::request::{HttpRequest, Methods, Routes};
use crate::route::favicon::favicon;
use crate::route::home::home;
use crate::route::login::{login, login_with_body};
use crate::session::AppState;
use crate::users;
use async_std::net::TcpStream;

mod dashboard;
mod favicon;
mod home;
mod login;

pub async fn handle_route(
    http_req: &HttpRequest,
    stream: &mut TcpStream,
    state: &AppState,
) -> Result<(), AppError> {
    match &http_req.header.method {
        Methods::Post => handle_post(&http_req, stream, state).await,
        Methods::Get => handle_get(&http_req, stream, state).await,
        _ => Err(HttpError::UnexpectedRoute("Wrong route".into()))?,
    }
}

async fn handle_post(
    http_req: &HttpRequest,
    stream: &mut TcpStream,
    state: &AppState,
) -> Result<(), AppError> {
    match http_req.header.route {
        Routes::Login => {
            let session = users::authenticate(&http_req, state).await;
            login_with_body(stream, session).await?;
            Ok(())
        }
        _ => Err(HttpError::UnexpectedRoute("Routes error".into()))?,
    }
}

async fn handle_get(
    req: &HttpRequest,
    stream: &mut TcpStream,
    state: &AppState,
) -> Result<(), AppError> {
    match req.header.route {
        Routes::Home => home(stream, state, req).await,
        Routes::Login => login(stream).await,
        Routes::Favicon => favicon(stream).await,
        _ => Err(HttpError::UnexpectedRoute("Routes error".into()))?,
    }
}
