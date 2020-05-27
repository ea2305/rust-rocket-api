use crate::app;
use crate::config;
use rocket;

pub fn create_routes() {
  rocket::ignite()
    .manage(config::database::init_pool())
    .mount("/posts",
      routes![
        app::controllers::posts::index
      ],
    ).launch();
}