use crate::config::database::DbConn;
use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::app::models::post::Post;
use crate::app::models::post::InsertablePost;
use crate::app::services;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Post>>, Status> {
    services::post::all(&connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

pub fn error_status(error: Error) -> Status {
    print!("{}", error.to_string());
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Post>, Status> {
    services::post::get(id, &connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<insertable_post>")]
pub fn post(insertable_post: Json<InsertablePost>, connection: DbConn) -> Result<status::Created<Json<Post>>, Status> {
    services::post::insert(insertable_post.into_inner(), &connection)
        .map(|post| post_created(post))
        .map_err(|error| error_status(error))
}

pub fn post_created(post: Post) -> status::Created<Json<Post>> {
    status::Created(format!("Success").to_string(), Some(Json(post)))
}

#[put("/<id>", format = "application/json", data = "<insertable_post>")]
pub fn put(id: i32, insertable_post: Json<InsertablePost>, connection: DbConn) -> Result<Json<Post>, Status> {
    services::post::update(id, insertable_post.into_inner(), &connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    match services::post::get(id, &connection) {
        Ok(_) => services::post::delete(id, &connection)
            .map(|_| status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}