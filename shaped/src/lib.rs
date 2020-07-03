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
