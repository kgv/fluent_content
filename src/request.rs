use crate::utils::default;
use fluent::{types::FluentNumber, FluentArgs, FluentValue};
use fmt::Write;
use std::{
    borrow::Borrow,
    fmt::{self, Display, Formatter},
};

fn parse_args(args: &str) -> FluentArgs {
    let mut fluent_args = FluentArgs::new();
    for arg in args.split('&') {
        match arg.split_once('=') {
            Some((key, value)) => match value {
                "" => fluent_args.set(key, FluentValue::None),
                // value if let Ok(value) = value.parse::<FluentNumber>() => fluent_args.set(key, value),
                value => match value.parse::<FluentNumber>() {
                    Ok(value) => fluent_args.set(key, value),
                    _ => fluent_args.set(key, value),
                },
            },
            None => fluent_args.set(arg.trim_end_matches('='), FluentValue::Error),
        }
    }
    fluent_args
}

/// Message content request
///
/// Provides access to a message content. Attribute and arguments are optional.
///
/// # Examples
///
/// Only identifier:
///
/// ```
/// # use fluent_content::Request;
/// #
/// let request = Request::from("id");
/// ```
///
/// Identifier and attribute:
///
/// ```
/// # use fluent_content::Request;
/// #
/// let request = Request::from("id.attr");
/// ```
///
/// Identifier and argument:
///
/// ```
/// # use fluent_content::Request;
/// #
/// let request = Request::from("id?key=value");
/// ```
///
/// Identifier, attribute and arguments:
///
/// ```
/// # use fluent_content::Request;
/// #
/// let request = Request::from("id.attr?key1=value1&key2=value2");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Request<'a, T> {
    pub id: &'a str,
    pub attr: Option<&'a str>,
    pub args: Option<T>,
}

impl<'a> Request<'a, &'static FluentArgs<'static>> {
    pub fn new(id: &'a str) -> Self {
        Self { id, ..default() }
    }
}

impl<'a, T> Request<'a, T> {
    pub fn attr(self, attr: &'a str) -> Self {
        Self {
            attr: Some(attr),
            ..self
        }
    }
}

impl<'a, T> Request<'a, T> {
    pub fn args<U>(self, args: U) -> Request<'a, U> {
        Request {
            id: self.id,
            attr: self.attr,
            args: Some(args),
        }
    }
}

impl<T> Default for Request<'_, T> {
    fn default() -> Self {
        Self {
            id: default(),
            attr: default(),
            args: default(),
        }
    }
}

impl<'a, T: Borrow<FluentArgs<'a>>> Display for Request<'_, T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, r#""{}"#, self.id)?;
        if let Some(attribute) = &self.attr {
            write!(f, ".{attribute}")?;
        }
        if let Some(args) = &self.args {
            let mut args = args.borrow().iter().peekable();
            if args.peek().is_some() {
                f.write_char('?')?;
            }
            for (key, value) in args {
                write!(f, "{key}=")?;
                match value {
                    FluentValue::String(value) => write!(f, "{value}")?,
                    FluentValue::Number(value) => write!(f, "{}", value.as_string())?,
                    FluentValue::Error => write!(f, "�")?,
                    FluentValue::None => {}
                    _ => unimplemented!(), // TODO!()
                }
            }
        }
        f.write_char('"')?;
        Ok(())
    }
}

impl<'a> From<&'a String> for Request<'a, FluentArgs<'a>> {
    fn from(value: &'a String) -> Self {
        Self::from(&**value)
    }
}

impl<'a> From<&'a str> for Request<'a, FluentArgs<'a>> {
    fn from(value: &'a str) -> Self {
        match value.split_once('.') {
            Some((id, value)) => match value.split_once('?') {
                Some((attr, args)) => Self {
                    id,
                    attr: Some(attr),
                    args: Some(parse_args(args)),
                },
                None => Self {
                    id,
                    attr: Some(value),
                    ..default()
                },
            },
            None => match value.split_once('?') {
                Some((id, args)) => Self {
                    id,
                    args: Some(parse_args(args)),
                    ..default()
                },
                None => Self {
                    id: value,
                    ..default()
                },
            },
        }
    }
}
