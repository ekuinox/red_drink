use clap::{App, SubCommand, Arg, ArgMatches};

use crate::subcommands::HuaSubCommand;
use crate::with_connection;

/// user command root
pub struct UserCommand;
/// create user command
struct CreateCommand;
/// show all user command
struct AllCommand;
/// add role to user command
struct AddRoleCommand;

pub const USER_COMMAND_NAME: &'static str = "user";
const CREATE_COMMAND_NAME: &'static str = "create";
const ALL_COMMAND_NAME: &'static str = "all";
const ADD_ROLE_COMMAND_NAME: &'static str = "add-role";

/// for management users
impl HuaSubCommand for UserCommand {
    fn create_subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(USER_COMMAND_NAME)
            .about("user management")
            .subcommand(CreateCommand::create_subcommand())
            .subcommand(AllCommand::create_subcommand())
            .subcommand(AddRoleCommand::create_subcommand())
    }
    fn run(matches: &ArgMatches) -> String {
        match matches.subcommand() {
            (CREATE_COMMAND_NAME, Some(create_command)) => CreateCommand::run(create_command),
            (ALL_COMMAND_NAME, Some(all_cmd)) => AllCommand::run(all_cmd),
            (ADD_ROLE_COMMAND_NAME, Some(add_role_command)) => AddRoleCommand::run(add_role_command),
            _ => "".to_string()
        }
    }
}

impl HuaSubCommand for CreateCommand {
    fn create_subcommand<'a, 'b>() -> App<'a, 'b>
    where 'a: 'b {
        SubCommand::with_name(CREATE_COMMAND_NAME)
        .about("create user")
        .arg(Arg::with_name("github-id")
            .long("github-id")
            .value_name("GitHub ID")
            .help("Your GitHub account's user id. not username")
            .required(true)
        )
    }
    fn run(matches: &ArgMatches) -> String {
        use red_drink::models::user::User;
        if let Some(github_user_id)  = matches.value_of("github-id").and_then(|id| id.parse::<i32>().ok()) {
            format!("{}", with_connection(|conn| {
                if let Some(user) = User::create_with_github_id(github_user_id, conn) {
                    format!("succeed to create user. user_id: {}", user.id)
                } else {
                    format!("failed to create user")
                }
            }))
        } else {
            format!("couldn't parse specified github id.")
        }
    }
}

impl HuaSubCommand for AllCommand {
    fn create_subcommand<'a, 'b>() -> App<'a, 'b>
    where 'a: 'b {
        SubCommand::with_name(ALL_COMMAND_NAME).about("show all users")
    }
    fn run(_: &ArgMatches) -> String {
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
}

impl HuaSubCommand for AddRoleCommand {
    fn create_subcommand<'a, 'b>() -> App<'a, 'b>
    where 'a: 'b {
        SubCommand::with_name(ADD_ROLE_COMMAND_NAME)
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
    }
    fn run(matches: &ArgMatches) -> String {
        use red_drink::models::user::User;
        if let (Some(user_id), Some(role_ids)) = (
            matches.value_of("user").and_then(|id| id.parse::<i32>().ok()) ,
            matches.values_of("role").map(|v| v.flat_map(|id| id.parse::<i32>().ok()).collect::<Vec<i32>>())
        ) {
            format!("{}", with_connection(|conn| {
                if let Some(user) = User::find(user_id, &conn) {
                    let results = role_ids.into_iter().map(|role_id| (user.add_role(role_id, &conn), role_id)).collect::<Vec<(bool, i32)>>();
                    let successes = results.iter().filter(|(ok, _)| *ok).map(|(_, id)| *id).collect::<Vec<i32>>();
                    let failures = results.iter().filter(|(ok, _)| !ok).map(|(_, id)| *id).collect::<Vec<i32>>();
                    format!("added roles: {:?}\nfailure ids: {:?}\n", successes, failures)
                } else {
                    "failed to find user".to_string()
                }
            }))
        } else {
            format!("user-id and role-id are required.")
        }
    }
}
