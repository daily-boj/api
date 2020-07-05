use crate::domain::MemberRepository;
use shaped::provider::VecProvider;
use std::sync::Arc;

pub fn create_member_id_provider(repo: Arc<MemberRepository>) -> VecProvider<String> {
    VecProvider::new(repo.get_all_user_id())
}
