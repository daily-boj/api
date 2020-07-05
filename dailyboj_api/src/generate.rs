use crate::domain::MemberRepository;
use crate::provider::*;
use crate::service::*;
use filedriver::Filedriver;
use shaped::prelude::*;
use std::path::Path;

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
        .generate_on(base)
}
