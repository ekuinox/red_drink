use clap::{App, ArgMatches};

pub trait HuaSubCommand {
    fn create_subcommand<'a, 'b>() -> App<'a, 'b> where 'a: 'b;
    fn run(matches: &ArgMatches) -> String;
}

pub mod user;
pub use user::*;

pub mod role;
pub use role::*;