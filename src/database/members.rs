use crate::models::member::{Member, MemberJson};
use crate::schema::members;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use serde::Serialize;
use uuid::Uuid;

pub enum MemberCreationError {
    DuplicateUsername,
    Generic,
}

impl From<Error> for MemberCreationError {
    fn from(error: Error) -> Self {
        match &error {
            Error::DatabaseError(DatabaseErrorKind::UniqueViolation, details) => {
                match details.constraint_name() {
                    Some("members_username_key") => MemberCreationError::DuplicateUsername,
                    _ => MemberCreationError::Generic,
                }
            }
            _ => MemberCreationError::Generic,
        }
    }
}

#[derive(Insertable)]
#[table_name = "members"]
pub struct NewMember {
    pub id: String,
    pub username: String,
}

#[derive(Serialize)]
pub struct MemberData {
    pub username: String,
}

pub fn create(conn: &PgConnection, data: MemberData) -> Result<MemberJson, MemberCreationError> {
    let new_member = NewMember {
        id: Uuid::new_v4().to_string(),
        username: data.username,
    };

    let result = diesel::insert_into(members::table)
        .values(new_member)
        .get_result::<Member>(conn);

    match result {
        Ok(member) => Ok(member.to_json()),
        Err(error) => Err(MemberCreationError::from(error)),
    }
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
