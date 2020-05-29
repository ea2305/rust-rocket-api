#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::posts;
use chrono::NaiveDateTime; // handle datetime with diesel

// Base schema
#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Create Schema
#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct InsertablePost {
    pub title: String,
    pub body: String
}
