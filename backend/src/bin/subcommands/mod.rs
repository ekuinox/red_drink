mod role;
mod user;

use clap::{App, ArgMatches};
use role::*;
use user::*;

pub trait HuaSubCommand {
    fn create_subcommand<'a, 'b>() -> App<'a, 'b> where 'a: 'b;
    fn run(matches: &ArgMatches) -> String;
}

pub fn create_subcommands<'a, 'b>() -> Vec<App<'a, 'b>> where 'a: 'b {
    vec![
        RoleCommand::create_subcommand(),
        UserCommand::create_subcommand(),
    ]
}

pub fn run(matches: &ArgMatches) -> String {
    match matches.subcommand() {
        (USER_COMMAND_NAME, Some(command)) => UserCommand::run(command),
        (ROLE_COMMAND_NAME, Some(command)) => RoleCommand::run(command),
        _ => "".to_string()
    }
}
