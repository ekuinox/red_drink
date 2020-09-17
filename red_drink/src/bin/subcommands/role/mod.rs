mod all;

use clap::{App, SubCommand, ArgMatches};
use crate::subcommands::HuaSubCommand;
use all::*;

/// for management roles
pub struct RoleCommand;
pub const ROLE_COMMAND_NAME: &'static str = "role";

impl HuaSubCommand for RoleCommand {
    fn create_subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(ROLE_COMMAND_NAME)
            .about("role management")
            .subcommand(AllCommand::create_subcommand())
    }
    fn run(matches: &ArgMatches) -> String {
        match matches.subcommand() {
            (ALL_COMMAND_NAME, Some(cmd)) => AllCommand::run(cmd),
            _ => "".to_string()
        }
    }
}
