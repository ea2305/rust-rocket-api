use diesel;
use diesel::prelude::*;

use crate::schema::posts;
use crate::app::models::post::Post;
use crate::app::models::post::InsertablePost;

pub fn all(connection: &MysqlConnection) -> QueryResult<Vec<Post>> {
    posts::table.load::<Post>(&*connection)
}

pub fn get(id: i32, connection: &MysqlConnection) -> QueryResult<Post> {
    posts::table.find(id).get_result::<Post>(connection)
}

pub fn insert(post: Post, connection: &MysqlConnection) -> QueryResult<usize> {
    diesel::insert_into(posts::table)
        .values(&InsertablePost::from_post(post))
        .execute(connection)
}

pub fn update(id: i32, post: Post, connection: &MysqlConnection) -> QueryResult<usize> {
    diesel::update(posts::table.find(id))
        .set(&post)
        .execute(connection)
}

pub fn delete(id: i32, connection: &MysqlConnection) -> QueryResult<usize> {
    diesel::delete(posts::table.find(id))
        .execute(connection)
}