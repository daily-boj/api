use nom::{error::ErrorKind as NomErrorKind, Err as NomError};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// The part of [`RoutePath`].
///
/// [`RoutePath`]: ../struct.RoutePath.html
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum RoutePathPart {
    /// Constant part. for example, `members`.
    Constant(String),
    /// Variable part. for example, `:id`.
    Variable(String),
}

/// API route path.
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub struct RoutePath {
    parts: Vec<RoutePathPart>,
}

/// The error returned when formatting route path failed.
#[derive(Debug, Error)]
pub enum RoutePathFormatError {
    /// A variable missed.
    #[error("Missing variable '{0}'")]
    MissingVariable(String),
}

impl fmt::Display for RoutePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted: Vec<String> = self
            .parts
            .iter()
            .map(|part| match part {
                RoutePathPart::Constant(path) => path.clone(),
                RoutePathPart::Variable(name) => format!(":{}", name),
            })
            .collect();
        f.write_str(&formatted.join("/"))
    }
}

impl RoutePath {
    /// Create RoutePath from raw parts.
    pub fn from_raw_parts(parts: Vec<RoutePathPart>) -> RoutePath {
        RoutePath { parts }
    }

    /// Format RoutePath with supplied variables
    pub fn format(
        &self,
        variables: HashMap<String, String>,
    ) -> Result<String, RoutePathFormatError> {
        let result: Result<Vec<_>, _> = self
            .parts
            .iter()
            .map(|part| match part {
                RoutePathPart::Constant(path) => Ok(path.clone()),
                RoutePathPart::Variable(name) => variables
                    .get(name)
                    .cloned()
                    .ok_or_else(|| RoutePathFormatError::MissingVariable(name.clone())),
            })
            .collect();
        Ok(result?.join("/"))
    }

    /// Get declared variables in order.
    pub fn variables(&self) -> Vec<String> {
        self.parts
            .iter()
            .filter_map(|part| match part {
                RoutePathPart::Constant(_) => None,
                RoutePathPart::Variable(value) => Some(value.clone()),
            })
            .collect()
    }
}

/// The error returned when parsing a route path failed.
#[derive(Debug, Error)]
pub enum RoutePathParseError {
    /// A part that is unused on parsing.
    #[error("Unused part left: \"{0}\"")]
    UnusedPartLeft(String),
    /// An error occured when parsing.
    #[error("An error occured when parsing: \"{0}\"")]
    Nom(NomError<(String, NomErrorKind)>),
}

impl FromStr for RoutePath {
    type Err = RoutePathParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match crate::parse::parse_route(s) {
            Ok(("", path)) => Ok(path),
            Ok((left, _)) => Err(RoutePathParseError::UnusedPartLeft(left.to_owned())),
            Err(e) => Err(RoutePathParseError::Nom(e.to_owned())),
        }
    }
}
