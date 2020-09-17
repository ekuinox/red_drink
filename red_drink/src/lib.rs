#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;
#[macro_use] extern crate rocket;
extern crate serde_json;

pub mod auth;
pub mod db;
pub mod routes;
pub mod github;
pub mod models;
pub mod types;
mod schema;
