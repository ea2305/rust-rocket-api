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

pub fn insert(post: InsertablePost, connection: &MysqlConnection) -> QueryResult<Post> {
    let index = diesel::insert_into(posts::table)
        .values(post)
        .execute(connection)
        .unwrap() as i32;

    posts::table.find(index).get_result::<Post>(connection)
}

pub fn update(id: i32, post: Post, connection: &MysqlConnection) -> QueryResult<Post> {
    let index = diesel::update(posts::table.find(id))
        .set(&post)
        .execute(connection)
        .unwrap() as i32;
    posts::table.find(index).get_result::<Post>(connection)    
}

pub fn delete(id: i32, connection: &MysqlConnection) -> QueryResult<usize> {
    diesel::delete(posts::table.find(id))
        .execute(connection)
}