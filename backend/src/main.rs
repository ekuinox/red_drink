#![feature(proc_macro_hygiene, decl_macro)]

extern crate base64;
extern crate oauth2;
extern crate rand;
extern crate http;
extern crate rocket_contrib;
#[macro_use] extern crate rocket;
extern crate serde;
extern crate serde_json;

mod routes;
mod github;
mod types;

use rocket_contrib::serve::StaticFiles;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(types::Session::fairing())
        .mount("/", routes![routes::auth::request_token, routes::auth::authorize])
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../frontend/dist")))
        .launch();
}