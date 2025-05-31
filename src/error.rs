use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("IO error {0}")]
    Io(#[from] async_std::io::Error),

    #[error("Unexpected Route {0}")]
    UnexpectedRoute(String),

    #[error("Unexpected Request {0}")]
    UnexpectedRequest(String),
}
