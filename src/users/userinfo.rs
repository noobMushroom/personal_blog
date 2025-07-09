use crate::error::HttpError;
use crate::request::HttpRequest;
use crate::utils::extract_from_string;

#[derive(Debug)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
}

impl UserInfo {
    pub fn get_user_info(req: &HttpRequest) -> Result<Self, HttpError> {
        let body = req.get_body()?;
        let username = extract_from_string(body, "username")
            .ok_or_else(|| HttpError::UnexpectedRequest("username not found".into()))?;
        let password = extract_from_string(body, "password")
            .ok_or_else(|| HttpError::UnexpectedRequest("password not found".into()))?;
        Ok(Self { username, password })
    }
}
