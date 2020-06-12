pub struct UnitProvider;

impl super::Provider for UnitProvider {
    type Item = ();
    fn provide(&self) -> Vec<Self::Item> {
        vec![()]
    }
}
