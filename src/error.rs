use fluent::resolver::ResolverError;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid id {0}")]
    Id(String),
    #[error("Invalid attribute {id}.{attribute}")]
    Attribute { id: String, attribute: String },
    #[error("Invalid value {id}")]
    Value { id: String },
    #[error("Invalid argument:\n{0:?}")]
    Argument(Vec<ResolverError>),
}
