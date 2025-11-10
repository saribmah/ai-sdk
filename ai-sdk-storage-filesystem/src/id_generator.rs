use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Monotonic counter for IDs generated in the same millisecond
static COUNTER: AtomicU64 = AtomicU64::new(0);

/// ID Generator for filesystem storage with sortable, unique identifiers
pub struct IdGenerator;

impl IdGenerator {
    /// Generate a session ID with reverse-chronological ordering
    /// Format: ses_{inverted_timestamp+counter}{random_suffix}
    /// Example: ses_7fffffffffff_a1b2c3d4e5f6g7
    pub fn generate_session_id() -> String {
        let time_component = Self::generate_time_component(true); // descending=true
        let random_suffix = Self::generate_random_suffix(14);
        format!("ses_{}{}", time_component, random_suffix)
    }

    /// Generate a message ID with chronological ordering
    /// Format: msg_{timestamp+counter}{random_suffix}
    /// Example: msg_01234567890a_h8i9j0k1l2m3n4
    pub fn generate_message_id() -> String {
        let time_component = Self::generate_time_component(false); // descending=false
        let random_suffix = Self::generate_random_suffix(14);
        format!("msg_{}{}", time_component, random_suffix)
    }

    /// Generate a part ID with chronological ordering
    /// Format: prt_{timestamp+counter}{random_suffix}
    /// Example: prt_01234567890a_o5p6q7r8s9t0u1
    pub fn generate_part_id() -> String {
        let time_component = Self::generate_time_component(false); // descending=false
        let random_suffix = Self::generate_random_suffix(14);
        format!("prt_{}{}", time_component, random_suffix)
    }

    /// Generate time component: first 6 bytes (12 hex chars) encode timestamp+counter
    ///
    /// - Timestamp: Current milliseconds shifted left by 12 bits (×0x1000)
    /// - Counter: Monotonic counter for same-millisecond IDs
    /// - Descending: If true, applies bitwise NOT (~) for reverse ordering
    fn generate_time_component(descending: bool) -> String {
        // Get current timestamp in milliseconds
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;

        // Shift left by 12 bits to make room for counter (0-4095)
        let shifted_time = now << 12;

        // Get monotonic counter (wraps at 4096)
        let counter = COUNTER.fetch_add(1, Ordering::SeqCst) & 0xFFF;

        // Combine timestamp and counter
        let mut time_value = shifted_time | counter;

        // For descending order (sessions), invert all bits
        if descending {
            time_value = !time_value;
        }

        // Take first 6 bytes (48 bits) and format as 12 hex characters
        format!("{:012x}", time_value & 0xFFFFFFFFFFFF)
    }

    /// Generate random base62 suffix for additional uniqueness
    /// Base62 alphabet: 0-9, A-Z, a-z
    fn generate_random_suffix(length: usize) -> String {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        use rand::Rng;

        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..BASE62.len());
                BASE62[idx] as char
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_id_format() {
        let id = IdGenerator::generate_session_id();
        assert!(id.starts_with("ses_"));
        assert_eq!(id.len(), 4 + 12 + 14); // prefix + time + random = 30
    }

    #[test]
    fn test_message_id_format() {
        let id = IdGenerator::generate_message_id();
        assert!(id.starts_with("msg_"));
        assert_eq!(id.len(), 4 + 12 + 14);
    }

    #[test]
    fn test_part_id_format() {
        let id = IdGenerator::generate_part_id();
        assert!(id.starts_with("prt_"));
        assert_eq!(id.len(), 4 + 12 + 14);
    }

    #[test]
    fn test_message_ids_are_ascending() {
        let id1 = IdGenerator::generate_message_id();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let id2 = IdGenerator::generate_message_id();

        // Lexicographic comparison should show id1 < id2
        assert!(id1 < id2);
    }

    #[test]
    fn test_session_ids_are_descending() {
        let id1 = IdGenerator::generate_session_id();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let id2 = IdGenerator::generate_session_id();

        // Lexicographic comparison should show id1 > id2 (reverse order)
        assert!(id1 > id2);
    }

    #[test]
    fn test_monotonic_counter_for_same_millisecond() {
        // Generate multiple IDs rapidly
        let ids: Vec<String> = (0..10)
            .map(|_| IdGenerator::generate_message_id())
            .collect();

        // All IDs should be unique
        let unique_count = ids.iter().collect::<std::collections::HashSet<_>>().len();
        assert_eq!(unique_count, 10);

        // IDs should be in ascending order
        let mut sorted_ids = ids.clone();
        sorted_ids.sort();
        assert_eq!(ids, sorted_ids);
    }
}
