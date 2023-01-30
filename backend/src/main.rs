mod routes;
mod db; 
mod schema;

use routes::adventurer;
use db::AdventurersGuild;
use rocket::Config;
use std::{env, collections::HashMap};
use dotenvy::dotenv;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_sync_db_pools;

#[macro_use]
extern crate diesel;

#[launch]
fn app() -> _ {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let mut dbs = HashMap::new();
    let mut db_config = HashMap::new();
    let db_url = env::var("DATABASE_URL").expect("missing DATABASE_URL in .env file.");
    db_config.insert("url", db_url);
    dbs.insert("adventurers_guild", db_config);

    let figment = Config::figment()
        .merge(("databases", dbs));

    rocket::custom(figment)
        .attach(AdventurersGuild::fairing())
        .mount("/adventurer", adventurer::get_routes())
}
