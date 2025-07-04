use crate::error::AppError;
use crate::request::HttpRequest;
use crate::session::{AppState, Session};

pub struct AuthedRequest<'a> {
    pub session: Session,
    pub req: &'a HttpRequest,
    pub state: &'a AppState,
}

impl<'a> AuthedRequest<'a> {
    pub fn new(req: &'a HttpRequest, state: &'a AppState) -> Result<Self, AppError> {
        let session = req.require_session(state)?;
        Ok(Self {
            session,
            req,
            state,
        })
    }
}
