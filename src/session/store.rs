use crate::error::MutexErrors;
use crate::session::session::Session;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type SessionStore = Arc<Mutex<HashMap<String, Session>>>;

#[derive(Clone)]
pub struct AppState {
    pub sessions: SessionStore,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
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
}
