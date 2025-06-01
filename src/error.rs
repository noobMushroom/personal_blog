use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Io error {0}")]
    Io(#[from] async_std::io::Error),

    #[error("IO error at {path} : {source} ")]
    IoWithContext {
        source: async_std::io::Error,
        path: String,
    },

    #[error("Unexpected Route {0}")]
    UnexpectedRoute(String),

    #[error("Unexpected Request {0}")]
    UnexpectedRequest(String),

    #[error("Json Parse error {0}")]
    JsonParse(#[from] serde_json::Error),
}
