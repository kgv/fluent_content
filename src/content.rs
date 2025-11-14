use crate::Request;
use fluent::{FluentArgs, FluentResource, bundle::FluentBundle, memoizer::MemoizerKind};
use std::borrow::Borrow;
use tracing::warn;

/// Content
pub trait Content<'a, T: Into<Request<'a, U>>, U: Borrow<FluentArgs<'a>>> {
    /// Request message content
    fn content(&self, request: T) -> Option<String>;
}

impl<'a, T, U, V, W> Content<'a, T, U> for FluentBundle<V, W>
where
    T: Into<Request<'a, U>>,
    U: Borrow<FluentArgs<'a>>,
    V: Borrow<FluentResource>,
    W: MemoizerKind,
{
    fn content(&self, request: T) -> Option<String> {
        let request = request.into();
        let request = request.borrow();
        let message = self.get_message(request.id)?;
        let pattern = match request.attr {
            Some(key) => message.get_attribute(key)?.value(),
            None => message.value()?,
        };
        let mut errors = Vec::new();
        let content = self
            .format_pattern(
                pattern,
                request.args.as_ref().map(Borrow::borrow),
                &mut errors,
            )
            .to_string();
        for error in &errors {
            warn!(%error);
        }
        Some(content)
    }
}
