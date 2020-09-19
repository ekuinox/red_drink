#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

mod auth;
mod routes;

use rocket_contrib::serve::StaticFiles;
use dotenv::dotenv;
use red_drink::*;

fn main() {
    dotenv().ok();

    rocket::ignite()
        .manage(db::connect())
        .attach(types::Session::fairing())
        .mount("/", routes![routes::auth::login, routes::auth::authorize])
        .mount("/api", routes![routes::api::get])
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../frontend/dist")))
        .launch();
}
