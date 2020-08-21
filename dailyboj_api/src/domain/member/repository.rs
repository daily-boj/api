use super::Member;
use filedriver::{Filedriver, Row, Table};
use std::sync::Arc;

pub struct MemberRepository {
    table: Table<Member>,
}

impl MemberRepository {
    pub fn new(connection: &Filedriver) -> Arc<Self> {
        Arc::new(MemberRepository {
            table: connection.table("member"),
        })
    }

    pub fn get_profile<T: AsRef<str>>(&self, id: T) -> Option<Member> {
        self.table.read(id).map(Row::take)
    }

    pub fn create_profile<T>(&mut self, id: T, member: Member) -> Row<Member>
    where
        T: AsRef<str>,
    {
        self.table.create(id, member)
    }

    pub fn has<T>(&mut self, id: T) -> bool
    where
        T: AsRef<str>,
    {
        self.table.has(id)
    }

    pub fn get_all_user_id(&self) -> Vec<String> {
        self.table.read_all_pk().collect()
    }

    pub fn get_all_profile(&self) -> Vec<Member> {
        self.table.read_all().map(Row::take).collect()
    }
}
