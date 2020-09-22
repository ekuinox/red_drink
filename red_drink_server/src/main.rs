#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

mod auth;
mod routes;

use rocket_contrib::serve::StaticFiles;
use serde_json::Value;
use dotenv::dotenv;
use red_drink::*;

pub type Session<'a> = rocket_session::Session<'a, serde_json::Map<String, Value>>;

fn main() {
    dotenv().ok();

    rocket::ignite()
        .manage(db::connect())
        .attach(Session::fairing())
        .mount("/", routes![routes::auth::login, routes::auth::authorize])
        .mount("/api", routes![
            routes::api::user::get, routes::api::user::get_user_by_username
        ])
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../frontend/dist")))
        .launch();
}
