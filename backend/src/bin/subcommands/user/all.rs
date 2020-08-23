use clap::{App, SubCommand, ArgMatches};
use crate::with_connection;
use crate::subcommands::HuaSubCommand;

/// to show all user
pub struct AllCommand;
pub const ALL_COMMAND_NAME: &'static str = "all";

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
