use crate::error::AppError;
use serde::Deserialize;

mod authenticate;
mod role;
mod userinfo;

pub use authenticate::*;
pub use role::*;
#[derive(Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub uuid: String,
    pub role: String,
    pub password: String,
}

pub struct Users {
    pub users: Vec<User>,
}

impl Users {
    pub async fn get_users() -> Result<Self, AppError> {
        let file = async_std::fs::read_to_string("Users/users.json").await?;
        let users: Vec<User> = serde_json::from_str(&file)?;
        Ok(Self { users })
    }
}

impl User {}
