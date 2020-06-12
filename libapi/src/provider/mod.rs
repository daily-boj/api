pub trait Provider {
    type Item;
    fn provide(&self) -> Vec<Self::Item>;
}

mod unit_provider;

pub use unit_provider::UnitProvider;
