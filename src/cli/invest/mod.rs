mod info;
mod table;

use self::info::{execute_invest_info_at, invest_info_subcommand, SUB_INVEST_INFO_AT};
use self::table::{execute_invest_table, invest_table_subcommand, SUB_INVEST_TABLE};
use clap::{App, Arg, ArgMatches, SubCommand};
use investment::Investment;

/// The invest sub command string
pub const SUB_INVEST: &str = "invest";
const ARG_PERIODICITY: &str = "period";
const ARG_YIELD_RATE: &str = "yield";
const ARG_CAPITAL: &str = "capital";
const ARG_REGULAR_ADDITION: &str = "addition";

/// Returns the loan sub command
pub fn invest_sub_commands<'a, 'b>() -> Vec<App<'a, 'b>> {
    let invest_sub_commands = vec![invest_info_subcommand(), invest_table_subcommand()];
    let sub_commands = vec![SubCommand::with_name(SUB_INVEST).subcommands(invest_sub_commands)];
    sub_commands
}

/// Execute the loan sub command
///
/// # Arguments
/// * `matches` - The command matches to retrieve the paramters
pub fn execute_invest_sub_command<'a>(matches: &ArgMatches<'a>) {
    match matches.subcommand() {
        (SUB_INVEST_INFO_AT, Some(info_at_matches)) => execute_invest_info_at(info_at_matches),
        (SUB_INVEST_TABLE, Some(table_matches)) => execute_invest_table(table_matches),
        _ => println!("*** No command found"),
    }
}

/// Return the common arguments for a loan
pub fn common_invest_args<'a, 'b>() -> Vec<Arg<'a, 'b>> {
    vec![
        Arg::with_name(ARG_PERIODICITY)
            .long(ARG_PERIODICITY)
            .short("p")
            .takes_value(true)
            .required(true)
            .help("perodicity for the loan (by year)"),
        Arg::with_name(ARG_YIELD_RATE)
            .long(ARG_YIELD_RATE)
            .short("r")
            .takes_value(true)
            .required(true)
            .help("perodicity for the loan (by year)"),
        Arg::with_name(ARG_CAPITAL)
            .long(ARG_CAPITAL)
            .short("c")
            .takes_value(true)
            .required(true)
            .help("capital to borrow"),
        Arg::with_name(ARG_REGULAR_ADDITION)
            .long(ARG_REGULAR_ADDITION)
            .short("a")
            .takes_value(true)
            .required(false)
            .default_value("0")
            .help("regular addition"),
    ]
}

/// Parse the common investment arguments from the cli
///
/// # Arguments
/// *  `matches` - cli arguments matches
pub fn parse_common_invest_args<'a>(matches: &ArgMatches<'a>) -> Investment {
    Investment::new(
        matches
            .value_of(ARG_CAPITAL)
            .unwrap()
            .parse::<u32>()
            .unwrap(),
        matches
            .value_of(ARG_PERIODICITY)
            .unwrap()
            .parse::<u8>()
            .unwrap(),
        matches
            .value_of(ARG_YIELD_RATE)
            .unwrap()
            .parse::<f32>()
            .unwrap()
            / 100_f32,
        matches
            .value_of(ARG_REGULAR_ADDITION)
            .unwrap()
            .parse::<u32>()
            .unwrap(),
    )
}
