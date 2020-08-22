extern crate clap;
extern crate red_drink;

use clap::{App, Arg, SubCommand};
use red_drink::db;

fn with_connection<F>(f: F) -> String where F: FnOnce(&db::DBConnection) -> String {
    match db::connect().get() {
        Ok(conn) => f(&conn),
        Err(err) => format!("failed to connect database. {:?}", err)
    }
}
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
        if let Some(users) = User::all_with_github(&connection) {
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

fn add_roles_to_user(user_id: i32, role_ids: Vec<i32>) -> String {
    use red_drink::models::user::User;
    with_connection(|conn| {
        if let Some(user) = User::find(user_id, &conn) {
            let results = role_ids.into_iter().map(|role_id| (user.add_role(role_id, &conn), role_id)).collect::<Vec<(bool, i32)>>();
            let successes = results.iter().filter(|(ok, _)| *ok).map(|(_, id)| *id).collect::<Vec<i32>>();
            let failures = results.iter().filter(|(ok, _)| !ok).map(|(_, id)| *id).collect::<Vec<i32>>();
            format!("added roles: {:?}\nfailure ids: {:?}\n", successes, failures)
        } else {
            "failed to find user".to_string()
        }
    })
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
            .subcommand(SubCommand::with_name("add-role")
                .about("add role to user")
                .arg(Arg::with_name("user")
                    .long("user-id")
                    .short("u")
                    .value_name("User ID")
                    .required(true)
                )
                .arg(Arg::with_name("role")
                    .long("role-id")
                    .short("r")
                    .value_name("Role ID")
                    .required(true)
                    .multiple(true)
                )
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
        if let Some(add_role_command) = user_command.subcommand_matches("add-role") {
            if let (Some(user_id), Some(role_ids)) = (
                add_role_command.value_of("user").and_then(|id| id.parse::<i32>().ok()) ,
                add_role_command.values_of("role").map(|v| v.flat_map(|id| id.parse::<i32>().ok()).collect::<Vec<i32>>())
            ) {
                println!("{}", add_roles_to_user(user_id, role_ids));
                return;
            }
        }
    }
    if let Some(role_command) = matches.subcommand_matches("role") {
        if let Some(_) = role_command.subcommand_matches("all") {
            println!("{}", show_all_roles());
            return;
        }
    }
}
