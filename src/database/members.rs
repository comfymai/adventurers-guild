use crate::models::member::{Member, MemberJson};
use crate::schema::members;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "members"]
pub struct NewMember<'a> {
    pub id: &'a str,
    pub username: &'a str,
}

#[derive(Serialize)]
pub struct MemberData<'a> {
    pub username: &'a str,
}

pub fn create<'a>(conn: &PgConnection, data: MemberData<'a>) -> MemberJson {
    let new_member = NewMember {
        id: &Uuid::new_v4().to_string()[..],
        username: data.username,
    };

    diesel::insert_into(members::table)
        .values(new_member)
        .get_result::<Member>(conn)
        .expect("failed to create member.")
        .to_json()
}

pub struct IndexingOptions {
    title: Option<String>,
    content: Option<String>,
    author_id: Option<String>,
    kind: Option<i32>,
}

pub fn index(conn: &PgConnection) -> Vec<Member> {
    members::table.load(conn).expect("failed to index members.")
}
