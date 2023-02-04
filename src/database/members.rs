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
    pub id: Option<String>,
    pub username: Option<String>,
}

pub fn index(conn: &PgConnection, options: IndexingOptions) -> Vec<MemberJson> {
    let mut query = members::table.into_boxed();

    if let Some(ref id) = options.id {
        query = query.filter(members::id.eq(id))
    }
    if let Some(ref username) = options.username {
        query = query.filter(members::username.eq(username))
    }

    query
        .load::<Member>(conn)
        .map(|result| result.into_iter().map(|member| member.to_json()).collect())
        .expect("failed to index members.")
}
