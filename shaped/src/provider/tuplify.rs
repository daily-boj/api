use super::Provider;

#[allow(non_snake_case)]
#[doc(hidden)]
pub struct Tuplified<ConcreteProvider: Provider>(ConcreteProvider);

impl<ConcreteProvider: Provider> Provider for Tuplified<ConcreteProvider> {
    type Item = (<ConcreteProvider as Provider>::Item,);

    fn provide(&self) -> Vec<Self::Item> {
        let mut items = Vec::new();
        for item in self.0.provide() {
            items.push((item,));
        }
        items
    }
}

pub trait Tuplify
where
    Self: Provider,
    Self: Sized,
{
    fn tuplify(self) -> Tuplified<Self>;
}

impl<ConcreteProvider: Provider> Tuplify for ConcreteProvider {
    fn tuplify(self) -> Tuplified<Self> {
        Tuplified(self)
    }
}
