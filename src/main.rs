#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod routes;

#[get("/")]
fn index() -> String {
    format!("Hello")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/posts", routes![routes::posts::index])
        .launch();
}