use crate::error::{AppError, MutexErrors};
use crate::session::session::Session;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tera::Tera;

pub type SessionStore = Arc<Mutex<HashMap<String, Session>>>;

#[derive(Clone)]
pub struct AppState {
    pub sessions: SessionStore,
    pub tempelates: Arc<Tera>,
}

impl AppState {
    pub fn new() -> Result<AppState, AppError> {
        let templates = Tera::new("html/**/*.html")?;
        Ok(Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            tempelates: Arc::new(templates),
        })
    }
    pub fn insert_session(&self, session: Session) -> Result<(), MutexErrors> {
        let mut store = self.sessions.lock()?;
        store.insert(session.uuid.clone(), session);
        Ok(())
    }

    pub fn get_session(&self, uuid: &str) -> Result<Option<Session>, MutexErrors> {
        let store = self.sessions.lock()?;
        Ok(store.get(uuid).cloned())
    }
    pub fn remove_session(&self, session_id: &str) -> Result<(), MutexErrors> {
        let mut store = self.sessions.lock()?;
        store.remove(session_id);
        Ok(())
    }

    pub fn validate_session(&self, session_id: &str) -> Result<bool, MutexErrors> {
        let store = self.sessions.lock()?;
        Ok(store.contains_key(session_id))
    }
}
