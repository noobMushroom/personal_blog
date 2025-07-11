use crate::error::{AppError, HttpError};
use crate::http::redirect_to_login;
use crate::request::authed::AuthedRequest;
use crate::request::{HttpRequest, Methods, Routes};
use crate::route::favicon::favicon;
use crate::route::home::home;
use crate::route::login::{login, login_with_body};
use crate::route::not_found::page_not_found;
use crate::session::AppState;
use crate::users;
use article_routes::article::article;
use article_routes::new::{add_article, get_article_html};
use article_routes::update_article::get_update_form;
use async_std::io::WriteExt;
use async_std::net::TcpStream;

mod article_routes;
mod dashboard;
mod favicon;
mod home;
mod login;
mod not_found;

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
    req: &HttpRequest,
    stream: &mut TcpStream,
    state: &AppState,
) -> Result<(), AppError> {
    match &req.header.route {
        Routes::Login => {
            let session = users::authenticate(&req, state).await;
            login_with_body(stream, session).await?;
            Ok(())
        }
        Routes::New => with_auth(req, state, stream, add_article).await,
        Routes::Unknown(_) => page_not_found(req, state, stream).await,
        _ => Err(HttpError::UnexpectedRoute("Routes error".into()))?,
    }
}

async fn handle_get(
    req: &HttpRequest,
    stream: &mut TcpStream,
    state: &AppState,
) -> Result<(), AppError> {
    match &req.header.route {
        Routes::Home => home(stream, state, req).await,
        Routes::Login => login(stream, state).await,
        Routes::Favicon => favicon(stream).await,
        Routes::New => with_auth(req, state, stream, get_article_html).await,
        Routes::Article(_) => article(req, state, stream).await,
        Routes::Update(_) => with_auth(req, state, stream, get_update_form).await,
        Routes::Unknown(_) => page_not_found(req, state, stream).await,
        _ => Err(HttpError::UnexpectedRoute("Routes error".into()))?,
    }
}

//Wrapper for the protected route
async fn with_auth<'a, F, Fut>(
    req: &'a HttpRequest,
    state: &'a AppState,
    stream: &'a mut TcpStream,
    handler: F,
) -> Result<(), AppError>
where
    F: FnOnce(AuthedRequest<'a>, &'a mut TcpStream) -> Fut,
    Fut: std::future::Future<Output = Result<(), AppError>> + 'a,
{
    match AuthedRequest::new(req, state) {
        Ok(authed_request) => handler(authed_request, stream).await,
        Err(_) => {
            let response = redirect_to_login();
            stream.write_all(response.as_bytes()).await?;
            Ok(())
        }
    }
}
