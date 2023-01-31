#[database("adventurers_guild")]
pub struct AdventurersGuild(diesel::PgConnection);

pub mod members;
