#![warn(clippy::all)]

mod action;
mod resolver;
mod service;
mod shaped;

pub mod openapi;
pub mod provider;
pub use crate::shaped::*;
pub use openapi::OpenApiSettings;
pub use provider::Provider;
pub use resolver::Resolver;
pub use route_path;
pub use service::Service;

pub use schemars;

pub mod prelude {
    pub use crate::openapi::{Info, OpenApiSettings, PathItem};
    pub use crate::provider;
    pub use crate::provider::{Provider, ProviderChain, Tuplify, UnitProvider, VecProvider};
    pub use crate::resolver::Resolver;
    pub use crate::service::Service;
    pub use crate::shaped::*;
}
