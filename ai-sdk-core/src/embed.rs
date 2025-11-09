/// Batch embedding generation.
pub mod many;
/// Result type for batch embedding operations.
pub mod many_result;
/// Result type for single embedding operations.
pub mod result;
/// Single embedding generation.
pub mod single;

pub use many::EmbedMany;
pub use many_result::{EmbedManyResult, EmbedManyResultResponseData};
pub use result::{EmbedResult, EmbedResultResponseData};
pub use single::Embed;
