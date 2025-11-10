//! File-based indexing system for fast lookups.

use ai_sdk_storage::MessageRole;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::error::FilesystemError;

/// Index for messages within a conversation session.
///
/// This index maintains the ordering and basic metadata of messages
/// to enable fast queries without reading all message files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageIndex {
    pub session_id: String,
    pub messages: Vec<MessageIndexEntry>,
}

/// Entry in the message index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageIndexEntry {
    pub message_id: String,
    pub created_at: DateTime<Utc>,
    pub role: MessageRole,
}

impl MessageIndex {
    /// Creates a new empty message index for a session.
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            messages: Vec::new(),
        }
    }

    /// Adds a message to the index.
    pub fn add_message(&mut self, entry: MessageIndexEntry) {
        self.messages.push(entry);
    }

    /// Gets message IDs in chronological order with optional limit.
    pub fn get_message_ids(&self, limit: Option<usize>) -> Vec<String> {
        let messages = if let Some(limit) = limit {
            &self.messages[self.messages.len().saturating_sub(limit)..]
        } else {
            &self.messages[..]
        };

        messages.iter().map(|e| e.message_id.clone()).collect()
    }

    /// Loads a message index from a file.
    pub async fn load(_path: &Path) -> Result<Self, FilesystemError> {
        todo!("MessageIndex::load")
    }

    /// Saves the message index to a file.
    pub async fn save(&self, _path: &Path) -> Result<(), FilesystemError> {
        todo!("MessageIndex::save")
    }
}

/// Index for all conversation sessions.
///
/// This global index enables fast session listing and filtering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionIndex {
    pub sessions: Vec<SessionIndexEntry>,
}

/// Entry in the session index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionIndexEntry {
    pub session_id: String,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message_count: usize,
}

impl SessionIndex {
    /// Creates a new empty session index.
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
        }
    }

    /// Adds a session to the index.
    pub fn add_session(&mut self, entry: SessionIndexEntry) {
        self.sessions.push(entry);
    }

    /// Removes a session from the index.
    pub fn remove_session(&mut self, session_id: &str) {
        self.sessions.retain(|s| s.session_id != session_id);
    }

    /// Updates session metadata in the index.
    pub fn update_session(
        &mut self,
        session_id: &str,
        updated_at: DateTime<Utc>,
        message_count: usize,
    ) {
        if let Some(entry) = self
            .sessions
            .iter_mut()
            .find(|s| s.session_id == session_id)
        {
            entry.updated_at = updated_at;
            entry.message_count = message_count;
        }
    }

    /// Gets sessions sorted by most recent, with optional limit.
    pub fn get_sessions(&self, limit: Option<usize>) -> Vec<SessionIndexEntry> {
        let mut sessions = self.sessions.clone();
        sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        if let Some(limit) = limit {
            sessions.truncate(limit);
        }

        sessions
    }

    /// Loads the session index from a file.
    pub async fn load(_path: &Path) -> Result<Self, FilesystemError> {
        todo!("SessionIndex::load")
    }

    /// Saves the session index to a file.
    pub async fn save(&self, _path: &Path) -> Result<(), FilesystemError> {
        todo!("SessionIndex::save")
    }
}

impl Default for SessionIndex {
    fn default() -> Self {
        Self::new()
    }
}
