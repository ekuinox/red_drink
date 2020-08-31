use clap::{App, SubCommand, Arg, ArgMatches};
use crate::with_connection;
use crate::subcommands::HuaSubCommand;

/// to create user
pub struct CreateCommand;
pub const CREATE_COMMAND_NAME: &'static str = "create";

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
        use red_drink::models::User;
        if let Some(github_user_id)  = matches.value_of("github-id").and_then(|id| id.parse::<i32>().ok()) {
            format!("{}", with_connection(|conn| {
                if let Ok(user) = User::create_with_github_id(github_user_id, conn) {
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
