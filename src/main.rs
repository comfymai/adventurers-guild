mod database;
mod models;
mod routes;
mod schema;

use database::AdventurersGuild;
use dotenvy::dotenv;
use rocket::{figment::Figment, Config, serde::json::{Value, serde_json::json}};
use routes::{members, posts};
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

#[catch(404)]
fn not_found() -> Value {
   json!({
       "message": "The requested resource could not be found or does not exist."
   }) 
}

#[catch(500)]
fn internal_error() -> Value {
    json!({
        "message": "An internal server error has occurred, please try again later."
    })
}

#[launch]
fn app() -> _ {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    rocket::custom(get_config())
        .attach(AdventurersGuild::fairing())
        .mount("/members", members::get_routes())
        .mount("/posts", posts::get_routes())
        .register("/", catchers![not_found, internal_error])
}
