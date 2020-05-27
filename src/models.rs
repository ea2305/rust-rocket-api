// handle datetime with diesel
use chrono::NaiveDateTime;

// Base schema
#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub crated_at: NaiveDateTime,
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
    fn from_post (post: Post) -> InsertablePost {
        InsertablePost {
            title: post.title,
            body: post.body,
        }
    }
}