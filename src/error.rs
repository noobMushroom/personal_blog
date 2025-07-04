use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Io error {0}")]
    Io(#[from] async_std::io::Error),

    #[error("IO error at {path} : {source} ")]
    IoWithContext {
        source: async_std::io::Error,
        path: String,
    },

    #[error("User Error: {0}")]
    User(#[from] UserError),

    #[error("Json Parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Mutex error: {0}")]
    Mutex(#[from] MutexErrors),

    #[error("Http error: {0}")]
    Http(#[from] HttpError),

    #[error("Cookie error: {0}")]
    Cookie(#[from] CookiesErrors),

    #[error("Session Error: {0}")]
    Session(#[from] SessionErrors),

    #[error("Html Parse error: {0}")]
    TeraError(#[from] tera::Error),
}

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Unexpected Route {0}")]
    UnexpectedRoute(String),

    #[error("Unexpected Request {0}")]
    UnexpectedRequest(String),
}

#[derive(Error, Debug)]
pub enum MutexErrors {
    #[error("Mutex was poisoned")]
    Poisoned,
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Invalid Credentials")]
    InvalidCredentials,
}

#[derive(Error, Debug)]
pub enum CookiesErrors {
    #[error("Missing session cookies")]
    MissingSessionCookies,
}

#[derive(Error, Debug)]
pub enum SessionErrors {
    #[error("Invalid session")]
    InvalidSession,
}

impl<T> From<std::sync::PoisonError<T>> for MutexErrors {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Self::Poisoned
    }
}
