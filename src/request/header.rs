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
    New,
    Article(ArticleRoutes),
    Delete(String),
    Update(String),
    Unknown(String),
}

pub enum ArticleRoutes {
    Article(String),
    Delete(String),
    Update(String),
    New,
}
pub struct Header {
    pub method: Methods,
    pub route: Routes,
    pub session_cookie: Option<String>,
}

fn parse_route(route: &str) -> Routes {
    match route {
        "/" => Routes::Home,
        "/login" => Routes::Login,
        "/favicon.ico" => Routes::Favicon,
        "/dashboard" => Routes::Dashboard,
        "/authenticate" => Routes::Authenticate,
        article if article.starts_with("/article") => parse_article_subdomain(article),
        unknown => Routes::Unknown(unknown.to_string()),
    }
}

fn parse_article_subdomain(path: &str) -> Routes {
    let link = path
        .trim_start_matches("/")
        .split('/')
        .collect::<Vec<&str>>();
    match link.as_slice() {
        ["article", "new"] => Routes::Article(ArticleRoutes::New),
        ["article", uuid] => Routes::Article(ArticleRoutes::Article(uuid.to_string())),
        ["article", "delete", uuid] => Routes::Article(ArticleRoutes::Delete(uuid.to_string())),
        ["article", "update", uuid] => Routes::Article(ArticleRoutes::Update(uuid.to_string())),
        _ => Routes::Unknown(path.to_string()),
    }
}

pub fn parse_cookie(req: &str) -> Option<String> {
    req.lines()
        .find(|line| line.starts_with("Cookie:"))
        .and_then(|value| {
            value.strip_prefix("Cookie: ").and_then(|cookie| {
                cookie.split("; ").find_map(|part| {
                    let mut parts = part.splitn(2, '=');
                    match (parts.next(), parts.next()) {
                        (Some("session_id"), Some(val)) => Some(val.to_string()),
                        _ => None,
                    }
                })
            })
        })
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

        let cookie = parse_cookie(req);

        let route = parse_route(
            parts
                .next()
                .ok_or_else(|| HttpError::UnexpectedRequest("Missing Route".into()))?,
        );

        Ok(Self {
            method,
            route,
            session_cookie: cookie,
        })
    }

    pub fn get_session_cookie(&self) -> Option<&str> {
        self.session_cookie.as_deref()
    }

    pub fn get_route_uuid(&self) -> &str {
        match &self.route {
            Routes::Article(article) => match article {
                ArticleRoutes::Update(ref uuid) => uuid,
                ArticleRoutes::Article(ref uuid) => uuid,
                ArticleRoutes::Delete(ref uuid) => uuid,
                _ => "",
            },
            _ => "",
        }
    }
}
