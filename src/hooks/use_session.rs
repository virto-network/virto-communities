use dioxus::prelude::*;
use gloo::storage::{errors::StorageError, LocalStorage};
use serde::{Deserialize, Serialize};

pub fn use_session() -> UseSessionState {
    let user = consume_context::<Signal<Option<UserSession>>>();

    use_hook(move || UseSessionState { data: user })
}

#[derive(Clone, Copy)]
pub struct UseSessionState {
    data: Signal<Option<UserSession>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserSession {
    pub name: String,
    pub account_id: u8,
}

#[derive(Debug)]
pub enum SessionError {
    SaveFailed,
    GetFailed,
}

impl UseSessionState {
    pub fn set(&mut self, data: &UserSession) {
        *self.data.write() = Some(data.clone());
    }

    pub fn get(&self) -> Option<UserSession> {
        self.data.read().clone()
    }

    pub fn persist_session_file(&self, session_file: &str) -> Result<(), SessionError> {
        <LocalStorage as gloo::storage::Storage>::set("session_file", session_file)
            .map_err(|_| SessionError::SaveFailed)
    }

    pub fn update_account(&mut self, account_id: u8) -> Result<(), SessionError> {
        let serialized_session: Result<String, StorageError> =
            <LocalStorage as gloo::storage::Storage>::get("session_file");

        let serialized_session = serialized_session.map_err(|_| SessionError::GetFailed)?;
        let mut full_session: UserSession =
            serde_json::from_str(&serialized_session).map_err(|_| SessionError::GetFailed)?;

        full_session.account_id = account_id;
        self.set(&full_session);

        let serialized_session =
            serde_json::to_string(&full_session).map_err(|_| SessionError::GetFailed)?;
        <LocalStorage as gloo::storage::Storage>::set("session_file", serialized_session)
            .map_err(|_| SessionError::SaveFailed)?;

        Ok(())
    }

    pub fn is_logged(&self) -> bool {
        let serialized_session: Result<String, StorageError> =
            <LocalStorage as gloo::storage::Storage>::get("session_file");

        let Ok(serialized_session) = serialized_session else {
            return false;
        };

        let Ok(_) = serde_json::from_str::<UserSession>(&serialized_session) else {
            return false;
        };

        true
    }
}
