mod info;

use self::info::{execute_invest_info_at, invest_info_subcommand, SUB_INVEST_INFO_AT};
use clap::{App, ArgMatches, SubCommand};

/// The loan sub command string
pub const SUB_INVEST: &str = "invest";

/// Returns the loan sub command
pub fn invest_sub_commands<'a, 'b>() -> Vec<App<'a, 'b>> {
    let loan_sub_commands = vec![invest_info_subcommand()];
    let sub_commands = vec![SubCommand::with_name(SUB_INVEST).subcommands(loan_sub_commands)];
    sub_commands
}

/// Execute the loan sub command
///
/// # Arguments
/// * `matches` - The command matches to retrieve the paramters
pub fn execute_invest_sub_command<'a>(matches: &ArgMatches<'a>) {
    match matches.subcommand() {
        (SUB_INVEST_INFO_AT, Some(info_at_matches)) => execute_invest_info_at(info_at_matches),
        _ => println!("*** No command found"),
    }
}
