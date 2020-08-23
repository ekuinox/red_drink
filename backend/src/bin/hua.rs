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

/// show all roles
fn show_all_roles() -> String {
    use red_drink::models::role::Role;
    with_connection(|conn| {
        if let Some(roles) = Role::all(&conn) {
            roles.into_iter().fold(vec![], |mut accumurator, role| {
                let mut about = "----------\n".to_string();
                about = about + &format!("id:\t{}\n", role.id);
                about = about + &format!("name:\t{}\n", role.name);
                let about = about + "----------\n";
                accumurator.push(about);
                accumurator
            }).join("\n")
        } else {
            format!("failed to get roles")
        }
    })
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
        ("all", Some(command)) => command.subcommand_matches("all").map(|_| show_all_roles()).unwrap_or("".to_string()),
        _ => "".to_string()
    };
    println!("{}", message);
}
