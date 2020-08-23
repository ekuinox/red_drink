use clap::{App, SubCommand, Arg, ArgMatches};
use red_drink::models::user::User;
use crate::with_connection;
use crate::subcommands::HuaSubCommand;

/// for permissions
pub struct PermissionCommand;
pub const PERMISSION_COMMAND_NAME: &'static str = "permission";

impl HuaSubCommand for PermissionCommand {
    fn create_subcommand<'a, 'b>() -> App<'a, 'b>
    where 'a: 'b {
        SubCommand::with_name(PERMISSION_COMMAND_NAME)
            .about("view user's permission")
            .arg(Arg::from_usage("user -u --user [User ID]").required(true))
            .arg(Arg::from_usage("has -h --has [Permission Path] 'Checks user has specific permission'").multiple(true))
            .arg(Arg::from_usage("all -a --all").conflicts_with("has")
        )
    }
    fn run(matches: &ArgMatches) -> String {
        // 上でrequiredかけてるからunwrapしちゃう
        let id = matches.value_of("user").and_then(|id| id.parse::<i32>().ok()).unwrap();
        with_connection(|conn| {
            if let Some(user) = User::find(id, conn) {
                if matches.is_present("all") {
                    user.get_permissions(conn)
                        .unwrap_or_default()
                        .into_iter()
                        .map(|permission| format!("{}\n", permission.path))
                        .collect::<String>()
                } else {
                    matches.values_of("has").map(
                        |values| values.into_iter().map(
                            |path| format!("{}: {}\n", path, user.has_permission(path.to_string(), conn))
                        ).collect::<String>()
                    ).unwrap_or("".to_string())
                }
            } else {
                format!("couldnot find user")
            }
        })
    }
}
