use clap::{App, ArgMatches, SubCommand};

/// The info module which contains the subcommand for info-at sub command
mod info;
/// The table module which contains the subcommand for table sub command
mod table;

use self::info::{execute_loan_info_at, loan_info_subcommand, SUB_LOAN_INFO_AT};
use self::table::{execute_loan_table, loan_table_subcommand, SUB_LOAN_TABLE};

/// The loan sub command string
pub const SUB_LOAN: &str = "loan";

/// Returns the loan sub command
pub fn loan_sub_command<'a, 'b, 'c>() -> Vec<App<'a, 'b>> {
    let loan_sub_commands = vec![loan_info_subcommand(), loan_table_subcommand()];
    let sub_commands = vec![SubCommand::with_name(SUB_LOAN).subcommands(loan_sub_commands)];
    sub_commands
}

/// Execute the loan sub command
///
/// # Arguments
/// * `matches` - The command matches to retrieve the paramters
pub fn execute_loan_sub_command<'a>(matches: &ArgMatches<'a>) {
    match matches.subcommand() {
        (SUB_LOAN_INFO_AT, Some(info_at_matches)) => execute_loan_info_at(info_at_matches),
        (SUB_LOAN_TABLE, Some(table_matches)) => execute_loan_table(table_matches),
        _ => println!("*** No command found"),
    }
}
