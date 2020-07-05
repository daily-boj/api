#![warn(clippy::all)]

mod action;
mod resolver;
mod service;
mod shaped;

pub mod provider;
pub use crate::shaped::*;
pub use provider::Provider;
pub use resolver::Resolver;
pub use route_path;
pub use service::Service;

pub mod prelude {
    pub use crate::provider;
    pub use crate::provider::Provider;
    pub use crate::provider::ProviderChain;
    pub use crate::provider::Tuplify;
    pub use crate::resolver::Resolver;
    pub use crate::service::Service;
    pub use crate::shaped::*;
}
