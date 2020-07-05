pub trait Provider: Sync + Send {
    type Item: Sized + Send;
    fn provide(&self) -> Vec<Self::Item>;
}

mod provider_chain;
mod tuplify;
pub use provider_chain::*;
pub use tuplify::*;

mod unit_provider;
mod vec_provider;
pub use unit_provider::UnitProvider;
pub use vec_provider::VecProvider;
