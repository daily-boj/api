pub struct VecProvider<T>(Vec<T>)
where
    T: Sized + Sync + Send + Clone;

impl<T> super::Provider for VecProvider<T>
where
    T: Sized + Sync + Send + Clone,
{
    type Item = T;
    fn provide(&self) -> Vec<Self::Item> {
        self.0.clone()
    }
}

impl<T> VecProvider<T>
where
    T: Sized + Sync + Send + Clone,
{
    pub fn new(elem: Vec<T>) -> Self {
        VecProvider(elem)
    }
}
