#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::posts;
use chrono::NaiveDateTime; // handle datetime with diesel

// Base schema
#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Create Schema
#[derive(Insertable)]
#[table_name = "posts"]
pub struct InsertablePost {
    pub title: String,
    pub body: String
}

impl InsertablePost {
    pub fn from_post (post: Post) -> InsertablePost {
        InsertablePost {
            title: post.title,
            body: post.body,
        }
    }
}