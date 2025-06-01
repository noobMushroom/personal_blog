use crate::error::HttpError;
use crate::route::favicon::favicon;
use crate::route::home::home;
use crate::route::login::login;
use async_std::net::TcpStream;

mod dashboard;
mod favicon;
mod home;
mod login;

pub enum Routes {
    Home,
    Login,
    Favicon,
}

pub fn parse_route(route: &str) -> Routes {
    match route {
        "/" => Routes::Home,
        "/login" => Routes::Login,
        "/favicon.ico" => Routes::Favicon,
        some => {
            println!("route parse: {}", some);
            unimplemented!()
        }
    }
}

pub async fn handle_route(route: &str, stream: &mut TcpStream) -> Result<(), HttpError> {
    let route = parse_route(route);
    match route {
        Routes::Home => home(stream).await,
        Routes::Login => {
            login(stream).await;
            Ok(())
        }
        Routes::Favicon => favicon(stream).await,
    }
}
