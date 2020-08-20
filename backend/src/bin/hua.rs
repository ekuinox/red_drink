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

/// show all users
fn show_all_users() -> String {
    use red_drink::models::user::User;
    if let Ok(connection) = db::connect().get() {
        if let Some(users) = User::all(&connection) {
            users.into_iter().fold(vec![], |mut accumurator, (user, github_user_opt)| {
                // create user's detail string
                let mut about = "----------\n".to_string();
                about = about + &format!("id:\t{}\n", user.id);
                if let Some(github_user) = github_user_opt {
                    about = about + &format!("github:\t{}\n", github_user.github_id);
                }
                let about = about + "----------\n";
                accumurator.push(about);
                accumurator
            }).join("\n")
        } else {
            format!("failed to get users")
        }
    } else {
        format!("failed to connect database")
    }
}

/// show all roles
fn show_all_roles() -> String {
    use red_drink::models::role::Role;
    if let Ok(conn) = db::connect().get() {
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
            .subcommand(SubCommand::with_name("all")
                .about("show all users")
            )
        )
        .subcommand(SubCommand::with_name("role")
            .about("role management")
            .subcommand(SubCommand::with_name("all")
                .about("show all roles")
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
        if let Some(_) = user_command.subcommand_matches("all") {
            println!("{}", show_all_users());
            return;
        }
    }
    if let Some(role_command) = matches.subcommand_matches("role") {
        if let Some(_) = role_command.subcommand_matches("all") {
            println!("{}", show_all_roles());
            return;
        }
    }
}
