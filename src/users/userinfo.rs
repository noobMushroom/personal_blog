use crate::error::HttpError;
use crate::request::HttpRequest;

#[derive(Debug)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
}

fn extract_from_string(body: &str, key: &str) -> Option<String> {
    body.split('&').find_map(|value| {
        let mut parts = value.splitn(2, '=');
        match (parts.next(), parts.next()) {
            (Some(k), Some(v)) if k == key => Some(v.to_string()),
            _ => None,
        }
    })
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
