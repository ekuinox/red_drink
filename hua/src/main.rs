#[macro_use]
extern crate clap;

mod subcommands;

use clap::{App};
use red_drink::db;
use subcommands::*;

/// use connection
pub fn with_connection<F>(f: F) -> String where F: FnOnce(&db::DBConnection) -> String {
    match db::connect().get() {
        Ok(conn) => f(&conn),
        Err(err) => format!("failed to connect database. {:?}", err)
    }
}

/// red_drink cli tool
fn main() {
    let matches = App::new(format!("hua <{}>", crate_name!()))
        .version(crate_version!())
        .author(crate_authors!())
        .about("red_drink server cli tools")
        .subcommands(create_subcommands())
        .get_matches();
    println!("{}", run(&matches));
}
