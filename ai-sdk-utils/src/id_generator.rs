use rand::Rng;
use std::sync::Arc;
use thiserror::Error;

/// Error type for ID generator creation.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum IdGeneratorError {
    /// The separator is part of the alphabet, which would cause ambiguity.
    #[error("The separator \"{separator}\" must not be part of the alphabet \"{alphabet}\".")]
    SeparatorInAlphabet { separator: String, alphabet: String },
}

/// A function that generates an ID.
///
/// This is a thread-safe function that can be called multiple times to generate
/// unique IDs. Each call returns a new randomly generated string.
pub type IdGenerator = Arc<dyn Fn() -> String + Send + Sync>;

/// Options for creating an ID generator.
///
/// # Examples
///
/// ```
/// use ai_sdk_utils::id_generator::IdGeneratorOptions;
///
/// // Default options
/// let options = IdGeneratorOptions::default();
///
/// // Custom options
/// let options = IdGeneratorOptions {
///     prefix: Some("user".to_string()),
///     size: 12,
///     alphabet: "0123456789ABCDEF".to_string(),
///     separator: "_".to_string(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct IdGeneratorOptions {
    /// Optional prefix for the generated IDs.
    pub prefix: Option<String>,

    /// The size of the random part of the ID. Default: 16.
    pub size: usize,

    /// The alphabet to use for the random part.
    /// Default: '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz'.
    pub alphabet: String,

    /// The separator between the prefix and random part. Default: '-'.
    pub separator: String,
}

impl Default for IdGeneratorOptions {
    fn default() -> Self {
        Self {
            prefix: None,
            size: 16,
            alphabet: "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".to_string(),
            separator: "-".to_string(),
        }
    }
}

impl IdGeneratorOptions {
    /// Creates a new `IdGeneratorOptions` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the prefix for the generated IDs.
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Sets the size of the random part of the ID.
    pub fn with_size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }

    /// Sets the alphabet to use for the random part.
    pub fn with_alphabet(mut self, alphabet: impl Into<String>) -> Self {
        self.alphabet = alphabet.into();
        self
    }

    /// Sets the separator between the prefix and random part.
    pub fn with_separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }
}

/// Creates an ID generator.
///
/// The total length of the ID is the sum of the prefix, separator, and random part length.
/// Not cryptographically secure.
///
/// # Arguments
///
/// * `options` - Configuration options for the ID generator.
///
/// # Returns
///
/// An `IdGenerator` function that can be called to generate IDs.
///
/// # Errors
///
/// Returns `Err` if the separator is part of the alphabet when a prefix is specified.
///
/// # Examples
///
/// ```
/// use ai_sdk_utils::id_generator::{create_id_generator, IdGeneratorOptions};
///
/// // Create a simple ID generator with default settings
/// let generator = create_id_generator(IdGeneratorOptions::default()).unwrap();
/// let id = generator();
/// assert_eq!(id.len(), 16);
///
/// // Create an ID generator with a prefix
/// let generator = create_id_generator(
///     IdGeneratorOptions::default()
///         .with_prefix("user")
///         .with_size(8)
/// ).unwrap();
/// let id = generator();
/// assert!(id.starts_with("user-"));
/// assert_eq!(id.len(), "user-".len() + 8);
///
/// // Create an ID generator with custom alphabet
/// let generator = create_id_generator(
///     IdGeneratorOptions::default()
///         .with_alphabet("0123456789ABCDEF")
///         .with_size(12)
/// ).unwrap();
/// let id = generator();
/// assert_eq!(id.len(), 12);
/// ```
pub fn create_id_generator(options: IdGeneratorOptions) -> Result<IdGenerator, IdGeneratorError> {
    let IdGeneratorOptions {
        prefix,
        size,
        alphabet,
        separator,
    } = options;

    // Create the base generator that generates random strings
    let base_generator = {
        let alphabet = alphabet.clone();
        move || {
            let mut rng = rand::thread_rng();
            let alphabet_bytes = alphabet.as_bytes();
            let alphabet_length = alphabet_bytes.len();

            (0..size)
                .map(|_| {
                    let idx = rng.gen_range(0..alphabet_length);
                    alphabet_bytes[idx] as char
                })
                .collect::<String>()
        }
    };

    // If no prefix, return the base generator
    if prefix.is_none() {
        return Ok(Arc::new(base_generator));
    }

    let prefix = prefix.unwrap();

    // Check that the separator is not part of the alphabet
    // (otherwise prefix checking can fail randomly)
    if alphabet.contains(&separator) {
        return Err(IdGeneratorError::SeparatorInAlphabet {
            separator,
            alphabet,
        });
    }

    // Return a generator that includes the prefix
    Ok(Arc::new(move || {
        format!("{}{}{}", prefix, separator, base_generator())
    }))
}

/// Generates a 16-character random string to use for IDs.
///
/// This is a convenience function that uses the default ID generator.
/// Not cryptographically secure.
///
/// # Examples
///
/// ```
/// use ai_sdk_utils::id_generator::generate_id;
///
/// let id1 = generate_id();
/// let id2 = generate_id();
///
/// assert_eq!(id1.len(), 16);
/// assert_eq!(id2.len(), 16);
/// assert_ne!(id1, id2); // IDs should be different
/// ```
pub fn generate_id() -> String {
    // Create a default generator once and reuse it
    // Note: We create it inline each time for simplicity, as the creation is cheap
    let generator = create_id_generator(IdGeneratorOptions::default())
        .expect("Default options should always be valid");
    generator()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let options = IdGeneratorOptions::default();
        assert_eq!(options.prefix, None);
        assert_eq!(options.size, 16);
        assert_eq!(
            options.alphabet,
            "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        );
        assert_eq!(options.separator, "-");
    }

    #[test]
    fn test_options_builder() {
        let options = IdGeneratorOptions::new()
            .with_prefix("user")
            .with_size(12)
            .with_alphabet("0123456789")
            .with_separator("_");

        assert_eq!(options.prefix, Some("user".to_string()));
        assert_eq!(options.size, 12);
        assert_eq!(options.alphabet, "0123456789");
        assert_eq!(options.separator, "_");
    }

    #[test]
    fn test_generate_id_default() {
        let id = generate_id();
        assert_eq!(id.len(), 16);

        // Check that all characters are from the default alphabet
        for c in id.chars() {
            assert!(c.is_ascii_alphanumeric());
        }
    }

    #[test]
    fn test_generate_id_uniqueness() {
        let id1 = generate_id();
        let id2 = generate_id();

        assert_ne!(id1, id2);
    }

    #[test]
    fn test_create_id_generator_no_prefix() {
        let generator = create_id_generator(IdGeneratorOptions::default()).unwrap();
        let id = generator();

        assert_eq!(id.len(), 16);
        for c in id.chars() {
            assert!(c.is_ascii_alphanumeric());
        }
    }

    #[test]
    fn test_create_id_generator_with_prefix() {
        let generator = create_id_generator(
            IdGeneratorOptions::default()
                .with_prefix("user")
                .with_size(8),
        )
        .unwrap();

        let id = generator();

        assert!(id.starts_with("user-"));
        assert_eq!(id.len(), "user-".len() + 8);
    }

    #[test]
    fn test_create_id_generator_custom_separator() {
        let generator = create_id_generator(
            IdGeneratorOptions::default()
                .with_prefix("item")
                .with_separator("_")
                .with_size(10),
        )
        .unwrap();

        let id = generator();

        assert!(id.starts_with("item_"));
        assert_eq!(id.len(), "item_".len() + 10);
    }

    #[test]
    fn test_create_id_generator_custom_alphabet() {
        let generator = create_id_generator(
            IdGeneratorOptions::default()
                .with_alphabet("0123456789ABCDEF")
                .with_size(12),
        )
        .unwrap();

        let id = generator();

        assert_eq!(id.len(), 12);
        for c in id.chars() {
            assert!("0123456789ABCDEF".contains(c));
        }
    }

    #[test]
    fn test_separator_in_alphabet_error() {
        let result = create_id_generator(
            IdGeneratorOptions::default()
                .with_prefix("user")
                .with_separator("a") // 'a' is in the default alphabet
                .with_size(8),
        );

        assert!(result.is_err());
        match result {
            Err(IdGeneratorError::SeparatorInAlphabet { separator, .. }) => {
                assert_eq!(separator, "a");
            }
            _ => panic!("Expected SeparatorInAlphabet error"),
        }
    }

    #[test]
    fn test_separator_not_in_alphabet_ok() {
        let result = create_id_generator(
            IdGeneratorOptions::default()
                .with_prefix("user")
                .with_separator("_")
                .with_alphabet("0123456789ABCDEF")
                .with_size(8),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_generator_called_multiple_times() {
        let generator = create_id_generator(IdGeneratorOptions::default()).unwrap();

        let id1 = generator();
        let id2 = generator();
        let id3 = generator();

        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_small_alphabet() {
        let generator = create_id_generator(
            IdGeneratorOptions::default()
                .with_alphabet("01")
                .with_size(20),
        )
        .unwrap();

        let id = generator();

        assert_eq!(id.len(), 20);
        for c in id.chars() {
            assert!(c == '0' || c == '1');
        }
    }

    #[test]
    fn test_size_zero() {
        let generator = create_id_generator(IdGeneratorOptions::default().with_size(0)).unwrap();

        let id = generator();

        assert_eq!(id.len(), 0);
    }

    #[test]
    fn test_size_one() {
        let generator = create_id_generator(IdGeneratorOptions::default().with_size(1)).unwrap();

        let id = generator();

        assert_eq!(id.len(), 1);
    }
}
