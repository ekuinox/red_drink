#![feature(proc_macro_hygiene, decl_macro)]

extern crate base64;
extern crate oauth2;
extern crate rand;
extern crate rocket_contrib;
#[macro_use] extern crate rocket;

mod routes;

use rocket_contrib::serve::StaticFiles;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    use routes::*;

    rocket::ignite()
        .mount("/", routes![get_token, auth])
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../frontend/dist")))
        .launch();
}