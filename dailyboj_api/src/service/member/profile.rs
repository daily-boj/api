use crate::domain::{Member, MemberRepository};
use service_macro::service;
use std::sync::Arc;

#[service("/member/profile/:id")]
pub fn profile(id: String, #[context] repo: Arc<MemberRepository>) -> Option<Member> {
    repo.get_profile(id)
}
