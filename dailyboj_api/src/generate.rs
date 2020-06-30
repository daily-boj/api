use crate::service::*;
use shaped::{self, provider, Shaped};
use std::path::Path;

pub fn generate<P: AsRef<Path>>(base: P) -> shaped::Result<()> {
    Shaped::new()
        .with(provider::UnitProvider, get_version)
        .generate_on(base)
}
