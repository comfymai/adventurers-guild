use crate::schema::posts;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Clone)]
pub struct Post {
    pub id: String,
    pub author_id: String,
    pub title: String,
    pub content: String,
    pub kind: i32,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub id: &'a str,
    pub author_id: &'a str,
    pub title: &'a str,
    pub content: &'a str,
    pub kind: i32,
}

#[derive(Serialize, Deserialize)]
pub struct PostData<'a> {
    pub author_id: &'a str,
    pub title: &'a str,
    pub content: &'a str,
    pub kind: i32,
}

pub fn create<'a>(conn: &PgConnection, data: PostData<'a>) -> Post {
    let new_post = NewPost {
        id: &Uuid::new_v4().to_string()[..],
        author_id: data.author_id,
        title: data.title,
        content: data.content,
        kind: data.kind
    };

    diesel::insert_into(posts::table)
        .values(new_post)
        .get_result::<Post>(conn)
        .expect("failed to create post.")
}
