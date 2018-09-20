use clap::{App, Arg, ArgMatches, SubCommand};

/// The info module which contains the subcommand for info-at sub command
mod info;
/// The table module which contains the subcommand for table sub command
mod table;

use self::info::{execute_loan_info_at, loan_info_subcommand, SUB_LOAN_INFO_AT};
use self::table::{execute_loan_table, loan_table_subcommand, SUB_LOAN_TABLE};
use loan::Loan;

/// The loan sub command string
pub const SUB_LOAN: &str = "loan";

const ARG_YEARS: &str = "years";
const ARG_PERIODICITY: &str = "periodicity";
const ARG_INTEREST_RATE: &str = "interest-rate";
const ARG_CAPITAL: &str = "capital";

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

/// Return the common arguments for a loan
pub fn common_loan_args<'a, 'b, 'c>() -> Vec<Arg<'b, 'c>> {
    vec![
        Arg::with_name(ARG_YEARS)
            .long(ARG_YEARS)
            .short("y")
            .takes_value(true)
            .required(true)
            .help("Number of years for the loan"),
        Arg::with_name(ARG_PERIODICITY)
            .long(ARG_PERIODICITY)
            .short("p")
            .takes_value(true)
            .required(true)
            .help("perodicity for the loan (by year)"),
        Arg::with_name(ARG_INTEREST_RATE)
            .long(ARG_INTEREST_RATE)
            .short("i")
            .takes_value(true)
            .required(true)
            .help("interest rate for the loan in percent"),
        Arg::with_name(ARG_CAPITAL)
            .long(ARG_CAPITAL)
            .short("c")
            .takes_value(true)
            .required(true)
            .help("capital to borrow"),
    ]
}

/// Return a loan from cli arguments
///
/// Arguments
/// * `matches` - The command matches to retrieve the paramters
pub fn parse_common_loan_args<'a>(matches: &ArgMatches<'a>) -> Loan {
    Loan::new(
        matches.value_of(ARG_YEARS).unwrap().parse::<u8>().unwrap(),
        matches
            .value_of(ARG_PERIODICITY)
            .unwrap()
            .parse::<u8>()
            .unwrap(),
        matches
            .value_of(ARG_INTEREST_RATE)
            .unwrap()
            .parse::<f32>()
            .unwrap()
            / 100_f32,
        matches
            .value_of(ARG_CAPITAL)
            .unwrap()
            .parse::<u32>()
            .unwrap(),
    )
}
