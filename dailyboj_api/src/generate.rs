use crate::domain::*;
use crate::provider::*;
use crate::service::*;
use filedriver::Filedriver;
use shaped::prelude::*;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn generate<P: AsRef<Path>>(base: P) -> Vec<RouteGeneration> {
    let connection = Filedriver::connect("resources/database")
        .expect("Cannot connect to the filedriver datbase.");
    let member_repository = MemberRepository::new(&connection);
    let member_id_provider = create_member_id_provider(member_repository.clone());

    Shaped::new()
        .with(provider::UnitProvider, meta::get_version)
        .with_context(
            member_repository.clone(),
            member_id_provider.tuplify(),
            member::profile,
        )
        .with_context(member_repository.clone(), UnitProvider, member::list)
        .generate_on(base)
}

macro_rules! schema {
    ($ty:ty) => {
        (stringify!($ty).to_owned(), schemars::schema_for!($ty))
    };
}

pub fn generate_schema<P: AsRef<Path>>(base: P) -> Vec<io::Result<PathBuf>> {
    let base = base.as_ref();
    if let Err(e) = fs::create_dir_all(base) {
        return vec![Err(e)];
    }
    let whole_schema = vec![schema!(Member)];

    whole_schema
        .iter()
        .map(|schema| {
            let target = base.join(format!("{}.schema.json", &schema.0));
            let generated = serde_json::to_string_pretty(&schema.1)?;
            std::fs::write(&target, generated)?;
            Ok(target)
        })
        .collect()
}
