use crate::error::HttpError;
use crate::route::home::home;
use crate::route::login::login;
use async_std::net::TcpStream;

mod dashboard;
mod home;
mod login;

pub enum Routes {
    Home,
    Login,
}

pub fn parse_route(route: &str) -> Routes {
    match route {
        "/" => Routes::Home,
        "/login" => Routes::Login,
        _ => unimplemented!(),
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
    }
}
