use clap::{App, SubCommand, Arg, ArgMatches};
use crate::with_connection;
use crate::subcommands::HuaSubCommand;

/// to add role to user
pub struct AddRoleCommand;
pub const ADD_ROLE_COMMAND_NAME: &'static str = "add-role";

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
