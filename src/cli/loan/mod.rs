use clap::{App, ArgMatches, SubCommand};

/// The info module whoch contains the subcommand for info-at sub command
mod info;

use self::info::{execute_loan_info_at, loan_info_subcommand, SUB_LOAN_INFO_AT};

/// The loan sub command string
pub const SUB_LOAN: &str = "loan";

/// Returns the loan sub command
pub fn loan_sub_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(SUB_LOAN).subcommand(loan_info_subcommand())
}

/// Execute the loan sub command
///
/// # Arguments
/// * `matches` - The command matches to retrieve the paramters
pub fn execute_loan_sub_command<'a>(matches: &ArgMatches<'a>) {
    match matches.subcommand() {
        (SUB_LOAN_INFO_AT, Some(info_at_matches)) => execute_loan_info_at(info_at_matches),
        _ => println!("*** No command found"),
    }
}
