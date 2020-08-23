use clap::{App, SubCommand, Arg, ArgMatches};

use crate::subcommands::HuaSubCommand;
use crate::with_connection;

pub struct UserCommand;

pub const USER_COMMAND_NAME: &'static str = "user";

/// for management users
impl HuaSubCommand for UserCommand {
    fn create_subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(USER_COMMAND_NAME)
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
    }
    fn run(matches: &ArgMatches) -> String {
        match matches.subcommand() {
            ("create", Some(create_command)) => {
                if let Some(github_user_id)  = create_command.value_of("github-id").and_then(|id| id.parse::<i32>().ok()) {
                    format!("{}", create_user(github_user_id))
                } else {
                    format!("couldn't parse specified github id.")
                }
            },
            ("all", Some(_)) => format!("{}", show_all_users()),
            ("add-role", Some(add_role_command)) => {
                if let (Some(user_id), Some(role_ids)) = (
                    add_role_command.value_of("user").and_then(|id| id.parse::<i32>().ok()) ,
                    add_role_command.values_of("role").map(|v| v.flat_map(|id| id.parse::<i32>().ok()).collect::<Vec<i32>>())
                ) {
                    format!("{}", add_roles_to_user(user_id, role_ids))
                } else {
                    format!("user-id and role-id are required.")
                }
            },
            _ => "".to_string()
        }
    }
}

/// create user with github user id
fn create_user(github_user_id: i32) -> String {
    use red_drink::models::user::User;
    with_connection(|conn| {
        if let Some(user) = User::create_with_github_id(github_user_id, conn) {
            format!("succeed to create user. user_id: {}", user.id)
        } else {
            format!("failed to create user")
        }
    })
}

/// show all users
fn show_all_users() -> String {
    use red_drink::models::user::User;
    with_connection(|conn| {
        if let Some(users) = User::all_with_github(conn) {
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
    })
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
