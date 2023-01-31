mod db;
mod routes;
mod schema;

use db::AdventurersGuild;
use dotenvy::dotenv;
use rocket::{figment::Figment, Config};
use routes::adventurer;
use std::{collections::HashMap, env};

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_sync_db_pools;

#[macro_use]
extern crate diesel;

fn get_config() -> Figment {
    let mut dbs = HashMap::new();
    let mut db_config = HashMap::new();
    let db_url = env::var("DATABASE_URL").expect("missing DATABASE_URL in .env file.");
    db_config.insert("url", db_url);
    dbs.insert("adventurers_guild", db_config);

    Config::figment().merge(("databases", dbs))
}

#[launch]
fn app() -> _ {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    rocket::custom(get_config())
        .attach(AdventurersGuild::fairing())
        .mount("/adventurer", adventurer::get_routes())
}
