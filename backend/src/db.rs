use crate::schema::adventurers;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use serde::Serialize;
use tracing::info;
use uuid::Uuid;

#[database("adventurers_guild")]
pub struct AdventurersGuild(diesel::PgConnection);

#[derive(Queryable, Serialize, Clone)]
pub struct Adventurer {
    pub id: String,
    pub username: String,
}

#[derive(Insertable)]
#[table_name = "adventurers"]
pub struct NewAdventurer {
    pub id: String,
    pub username: String,
}

pub fn create(conn: &PgConnection, username: String) -> Adventurer {
    info!("creating adventurer");
    let new_adventurer = NewAdventurer {
        id: Uuid::new_v4().to_string(),
        username,
    };

    diesel::insert_into(adventurers::table)
        .values(new_adventurer)
        .get_result::<Adventurer>(conn)
        .expect("failed to insert adventurer to database.")
}

pub fn index(conn : &PgConnection) -> Vec<Adventurer> {
    adventurers::table.load(conn)
        .expect("failed to index adventurers.")
}
