extern crate clap;
extern crate red_drink;

use clap::{App, Arg, SubCommand};
use red_drink::db;

/// create user with github user id
fn create_user(github_user_id: i32) -> String {
    use red_drink::models::user::User;
    if let Ok(connection) = db::connect().get() {
        if let Some(user) = User::create_with_github_id(github_user_id, &connection) {
            format!("succeed to create user. user_id: {}", user.id)
        } else {
            format!("failed to create user")
        }
    } else {
        format!("failed to connect database")
    }
}

/// red_drink cli tool
fn main() {
    let matches = App::new("hua")
        .version("0.0.1")
        .author("ekuinox <depkey@me.com>")
        .about("red_drink server cli tools")
        .subcommand(SubCommand::with_name("user")
            .about("user management")
            .subcommand(SubCommand::with_name("create")
                .about("create user")
                .arg(Arg::with_name("github-id")
                    .long("github-id")
                    .value_name("GitHub ID")
                    .help("Your GitHub account's user id. not username")
                    .required(true)
                )
            )
        )
        .get_matches();

    if let Some(user_command) = matches.subcommand_matches("user") {
        if let Some(create_command) = user_command.subcommand_matches("create") {
            if let Some(github_user_id)  = create_command.value_of("github-id").and_then(|id| id.parse::<i32>().ok()) {
                println!("{}", create_user(github_user_id));
            } else {
                println!("couldn't parse specified github id.");
            }
            return;
        }
    }
}
