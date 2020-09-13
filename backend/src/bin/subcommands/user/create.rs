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
        .arg(Arg::with_name("username")
            .short("n")
            .required(true)
        )
        .arg(Arg::with_name("avatar_url")
            .short("a")
            .required(true)
        )
        .arg(Arg::with_name("email")
            .short("e")
            .required(true)
        )
    }
    fn run(matches: &ArgMatches) -> String {
        use red_drink::models::{User, user::GitHubAccountDetail};
        if let (Some(id), Some(username), Some(avatar_url), Some(email)) = (
            matches.value_of("github-id").and_then(|id| id.parse::<i32>().ok()),
            matches.value_of("username").map(|username| username.to_string()),
            matches.value_of("avatar_url").map(|avatar_url| avatar_url.to_string()),
            matches.value_of("email").map(|email| email.to_string())
        ) {
            let detail = GitHubAccountDetail { id, login: username.clone(), avatar_url, email, name: username.clone() };
            format!("{}", with_connection(|conn| {
                if let Ok(user) = User::create_with_github_detail(detail, conn) {
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
