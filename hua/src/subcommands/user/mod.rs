mod add_role;
mod all;
mod create;

use clap::{App, SubCommand, ArgMatches};
use crate::subcommands::HuaSubCommand;
use add_role::*;
use all::*;
use create::*;

/// for management users
pub struct UserCommand;
pub const USER_COMMAND_NAME: &'static str = "user";

impl HuaSubCommand for UserCommand {
    fn create_subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(USER_COMMAND_NAME)
            .about("user management")
            .subcommands(vec![
                CreateCommand::create_subcommand(),
                AllCommand::create_subcommand(),
                AddRoleCommand::create_subcommand(),
            ])
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
