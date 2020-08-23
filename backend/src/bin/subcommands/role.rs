use clap::{App, SubCommand, ArgMatches};

use crate::subcommands::HuaSubCommand;
use crate::with_connection;

pub struct RoleCommand;

pub const ROLE_COMMAND_NAME: &'static str = "role";

impl HuaSubCommand for RoleCommand {
    fn create_subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(ROLE_COMMAND_NAME)
            .about("role management")
            .subcommand(SubCommand::with_name("all")
                .about("show all roles")
            )
    }
    fn run(matches: &ArgMatches) -> String {
        matches.subcommand_matches("all").map(|_| show_all_roles()).unwrap_or("".to_string())
    }
}
/// show all roles
fn show_all_roles() -> String {
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
