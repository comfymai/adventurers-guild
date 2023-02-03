use crate::models::post::Post;

use rocket::{serde::json::Json, Route};
use serde::{Deserialize, Serialize};

use crate::database::{posts::{self, PostData}, AdventurersGuild};

#[derive(Serialize)]
pub struct NewResponse {
    post: Post,
}

#[derive(Serialize)]
pub struct IndexResponse {
    posts: Vec<Post>
}

#[derive(Deserialize)]
pub struct NewData {
    pub author_id: String,
    pub title: String,
    pub content: String,
    pub kind: i32,
}

#[post("/new", data = "<data>")]
pub async fn new(data: Json<NewData>, db: AdventurersGuild) -> Json<NewResponse> {
    let post = db.run(move |conn| {
        posts::create(&conn, PostData {
            author_id: &data.author_id[..],
            title: &data.title[..],
            content: &data.content[..],
            kind: data.kind
        })
    }).await;

    Json(NewResponse { post })
}

#[get("/")]
pub async fn index(db: AdventurersGuild) -> Json<IndexResponse> {
    let posts = db.run(|conn| {
        posts::index(&conn)
    }).await;

    Json(IndexResponse { posts })
}

pub fn get_routes() -> Vec<Route> {
    routes![new, index]
}
