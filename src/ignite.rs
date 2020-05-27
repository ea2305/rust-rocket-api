use crate::app::controllers::posts;
use crate::config;
use rocket;

pub fn generate_routes() {
  rocket::ignite()
    .manage(config::database::init_pool())
    .mount("/posts",
      routes![
        posts::index
      ]
    )
    .launch();
}