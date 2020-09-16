#![feature(proc_macro_hygiene, decl_macro)]

extern crate base64;
extern crate oauth2;
extern crate rand;
extern crate http;
extern crate rocket_contrib;
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;
extern crate serde_json;

mod auth;
mod models;
mod db;
mod routes;
mod github;
mod types;
mod schema;

use rocket_contrib::serve::StaticFiles;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    rocket::ignite()
        .manage(db::connect())
        .attach(types::Session::fairing())
        .mount("/", routes![routes::auth::login, routes::auth::authorize])
        .mount("/api", routes![
            routes::api::user::get, routes::api::user::get_user_by_username
        ])
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../frontend/dist")))
        .launch();
}
