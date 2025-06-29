use crate::error::{AppError, UserError};
use crate::request::HttpRequest;
use crate::session;
use crate::session::{AppState, Session};
use crate::users::{userinfo, Roles, Users};

pub async fn authenticate(
    req: &HttpRequest,
    session_store: &AppState,
) -> Result<Session, AppError> {
    let user_info = userinfo::UserInfo::get_user_info(req)?;
    let users = Users::get_users().await?;
    let user = users
        .users
        .iter()
        .find(|user| user.username == user_info.username && user.password == user_info.password)
        .ok_or_else(|| UserError::InvalidCredentials)?;
    let session = session::Session::new(&user.uuid, &user.username, Roles::from(&user.role));
    session_store.insert_session(session.clone())?;
    Ok(session)
}
