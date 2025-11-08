pub mod many;
pub mod many_result;
pub mod result;
pub mod single;

pub use many::EmbedMany;
pub use many_result::{EmbedManyResult, EmbedManyResultResponseData};
pub use result::{EmbedResult, EmbedResultResponseData};
pub use single::Embed;
