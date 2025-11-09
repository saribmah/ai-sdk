//! AI SDK Utilities
//!
//! Shared utilities for the AI SDK workspace.

#![warn(missing_docs)]

/// Adds two numbers together.
///
/// This is a placeholder function for the utilities crate.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
