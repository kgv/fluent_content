use crate::{
    error::{Error, Result},
    Request,
};
use fluent::{
    bundle::FluentBundle, memoizer::MemoizerKind, FluentArgs, FluentError, FluentResource,
};
use std::borrow::Borrow;

/// Content
pub trait Content<'a, T: Into<Request<'a, U>>, U: Borrow<FluentArgs<'a>>> {
    /// Request message content
    fn content(&self, request: T) -> Result<String>;
}

impl<'a, T, U, V, W> Content<'a, T, U> for FluentBundle<V, W>
where
    T: Into<Request<'a, U>>,
    U: Borrow<FluentArgs<'a>>,
    V: Borrow<FluentResource>,
    W: MemoizerKind,
{
    fn content(&self, request: T) -> Result<String> {
        let request = request.into();
        let request = request.borrow();
        let message = self
            .get_message(request.id)
            .ok_or_else(|| Error::Id(request.id.to_string()))?;
        let pattern = match request.attr {
            Some(key) => message
                .get_attribute(key)
                .ok_or_else(|| Error::Attribute {
                    id: request.id.to_string(),
                    attribute: key.to_string(),
                })?
                .value(),
            None => message.value().ok_or_else(|| Error::Value {
                id: request.id.to_string(),
            })?,
        };
        let mut errors = Vec::new();
        let content = self
            .format_pattern(
                pattern,
                request.args.as_ref().map(Borrow::borrow),
                &mut errors,
            )
            .to_string();
        if !errors.is_empty() {
            let errors = errors
                .into_iter()
                .map(|error| match error {
                    FluentError::ResolverError(error) => error,
                    _ => unreachable!(),
                })
                .collect();
            return Err(Error::Argument(errors));
        }
        Ok(content)
    }
}
