#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;
extern crate serde_json;

pub mod db;
pub mod models;
mod schema;