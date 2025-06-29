use crate::users;
use crate::users::Roles;

#[derive(Clone)]
pub struct Session {
    pub uuid: String,
    pub user_id: String,
    pub username: String,
    pub role: Roles,
}

impl Session {
    pub fn new(user_id: &String, username: &String, role: users::Roles) -> Self {
        let user_id = user_id.clone();
        let username = username.clone();
        Self {
            uuid: Self::gen_session_id(),
            user_id,
            username,
            role,
        }
    }

    // Generates new session id for each session
    fn gen_session_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }
}
