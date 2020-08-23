mod subcommands;

use clap::{App, SubCommand};
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
    let matches = App::new("hua")
        .version("0.0.1")
        .author("ekuinox <depkey@me.com>")
        .about("red_drink server cli tools")
        .subcommand(UserCommand::create_subcommand())
        .subcommand(SubCommand::with_name("role")
            .about("role management")
            .subcommand(SubCommand::with_name("all")
                .about("show all roles")
            )
        )
        .get_matches();

    let message = match matches.subcommand() {
        (USER_COMMAND_NAME, Some(command)) => UserCommand::run(command),
        (ROLE_COMMAND_NAME, Some(command)) => RoleCommand::run(command),
        _ => "".to_string()
    };
    println!("{}", message);
}
