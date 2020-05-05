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

use rocket_contrib::serve::StaticFiles;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    use routes::*;

    rocket::ignite()
        .attach(Session::fairing())
        .mount("/", routes![get_token, auth])
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../frontend/dist")))
        .launch();
}