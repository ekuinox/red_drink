use clap::{App, SubCommand, ArgMatches};
use crate::subcommands::HuaSubCommand;
use crate::with_connection;

/// to show all roles
pub struct AllCommand;
pub const ALL_COMMAND_NAME: &'static str = "all";

impl HuaSubCommand for AllCommand {
    fn create_subcommand<'a, 'b>() -> App<'a, 'b>
    where 'a: 'b {
        SubCommand::with_name(ALL_COMMAND_NAME).about("show all roles")
    }
    fn run(_: &ArgMatches) -> String {
        use red_drink::models::role::Role;
        with_connection(|conn| {
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
        })
    }
}