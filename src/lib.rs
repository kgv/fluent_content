//! Fluent content
//!
//! Fluent extension for easy access to message content.

pub use self::{content::Content, request::Request};

/// `use fluent_content::prelude::*;` to import common traits
pub mod prelude {
    #[doc(inline)]
    pub use super::Content;
}

mod content;
mod request;
