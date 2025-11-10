//! Utility functions for working with storage.
//!
//! This module provides helper functions for generating unique IDs
//! and working with storage types.

/// Generates a unique session ID.
///
/// This function creates a new UUID v4 identifier that can be used
/// as a session ID for conversation storage.
///
/// # Example
///
/// ```rust
/// use ai_sdk_storage::utils::generate_session_id;
///
/// let session_id = generate_session_id();
/// println!("New session: {}", session_id);
/// ```
pub fn generate_session_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Generates a unique message ID.
///
/// This function creates a new UUID v4 identifier that can be used
/// as a message ID for conversation storage.
///
/// # Example
///
/// ```rust
/// use ai_sdk_storage::utils::generate_message_id;
///
/// let message_id = generate_message_id();
/// println!("New message: {}", message_id);
/// ```
pub fn generate_message_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_session_id() {
        let id1 = generate_session_id();
        let id2 = generate_session_id();

        // IDs should not be empty
        assert!(!id1.is_empty());
        assert!(!id2.is_empty());

        // IDs should be unique
        assert_ne!(id1, id2);

        // IDs should be valid UUIDs (36 characters with dashes)
        assert_eq!(id1.len(), 36);
        assert_eq!(id2.len(), 36);
    }

    #[test]
    fn test_generate_message_id() {
        let id1 = generate_message_id();
        let id2 = generate_message_id();

        // IDs should not be empty
        assert!(!id1.is_empty());
        assert!(!id2.is_empty());

        // IDs should be unique
        assert_ne!(id1, id2);

        // IDs should be valid UUIDs
        assert_eq!(id1.len(), 36);
        assert_eq!(id2.len(), 36);
    }

    #[test]
    fn test_id_format() {
        let session_id = generate_session_id();
        let message_id = generate_message_id();

        // Both should be valid UUID format (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx)
        assert!(session_id.contains('-'));
        assert!(message_id.contains('-'));

        // Should have 4 dashes
        assert_eq!(session_id.matches('-').count(), 4);
        assert_eq!(message_id.matches('-').count(), 4);
    }
}
