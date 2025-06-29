use crate::error::HttpError;

pub enum Methods {
    Get,
    Post,
    Unknown(String),
}

pub enum Routes {
    Home,
    Login,
    Favicon,
    Dashboard,
    Authenticate,
    Article(String),
    Unknown(String),
}
pub struct Header {
    pub method: Methods,
    pub route: Routes,
}

fn parse_route(route: &str) -> Routes {
    match route {
        "/" => Routes::Home,
        "/login" => Routes::Login,
        "/favicon.ico" => Routes::Favicon,
        "/dashboard" => Routes::Dashboard,
        "/authenticate" => Routes::Authenticate,
        some => {
            println!("route parse: {}", some);
            unimplemented!()
        }
    }
}

fn parse_method(method: &str) -> Methods {
    match method {
        "GET" => Methods::Get,
        "POST" => Methods::Post,
        unknown => Methods::Unknown(unknown.to_string()),
    }
}

impl Header {
    pub fn new(req: &str) -> Result<Header, HttpError> {
        let header = req
            .lines()
            .next()
            .ok_or_else(|| HttpError::UnexpectedRequest("Header Missing".into()))?;
        let mut parts = header.split_ascii_whitespace();
        let method = parse_method(
            parts
                .next()
                .ok_or_else(|| HttpError::UnexpectedRequest("Missing Method".into()))?,
        );

        let route = parse_route(
            parts
                .next()
                .ok_or_else(|| HttpError::UnexpectedRequest("Missing Route".into()))?,
        );

        Ok(Self { method, route })
    }
}
