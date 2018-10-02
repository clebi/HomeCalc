mod compare_at;

use self::compare_at::{execute_home_compare_at, home_compare_at_subcommand, SUB_HOME_COMPARE_AT};
use clap::{App, Arg, ArgMatches, SubCommand};
use comparators::HomeInvest;

pub const SUB_HOME: &str = "home";
const ARG_SUPPLY: &str = "supply";
const ARG_LOAN: &str = "loan";
const ARG_LOAN_RATE: &str = "loan-rate";
const ARG_PURCHASE_CHARGES: &str = "purchase-charges";
const ARG_ANNUAL_CHARGES: &str = "annual-charges";
const ARG_HOME_APPRECIATION: &str = "home-appreciation";
const ARG_HOME_RENT: &str = "rent";
const ARG_INVEST_RATE_RENT: &str = "invest-rate";
const ARG_YEARS: &str = "years";

/// Returns the home sub commands
pub fn home_sub_commands<'a, 'b>() -> Vec<App<'a, 'b>> {
    let home_sub_commands = vec![home_compare_at_subcommand()];
    let sub_commands = vec![SubCommand::with_name(SUB_HOME).subcommands(home_sub_commands)];
    sub_commands
}

/// Execute the home sub command
///
/// # Arguments
/// * `matches` - The command matches to retrieve the parameters
pub fn execute_home_sub_command<'a>(matches: &ArgMatches<'a>) {
    match matches.subcommand() {
        (SUB_HOME_COMPARE_AT, Some(matches)) => execute_home_compare_at(matches),
        _ => println!("*** No command found"),
    }
}

/// Return the common arguments for home
pub fn common_home_args<'a, 'b>() -> Vec<Arg<'a, 'b>> {
    vec![
        Arg::with_name(ARG_SUPPLY)
            .long(ARG_SUPPLY)
            .short("s")
            .takes_value(true)
            .required(true)
            .help("supply for the home purchase"),
        Arg::with_name(ARG_LOAN)
            .long(ARG_LOAN)
            .short("l")
            .takes_value(true)
            .required(true)
            .help("the loan value for home purchase"),
        Arg::with_name(ARG_LOAN_RATE)
            .long(ARG_LOAN_RATE)
            .short("r")
            .takes_value(true)
            .required(true)
            .help("the loan rate (everything included)"),
        Arg::with_name(ARG_PURCHASE_CHARGES)
            .long(ARG_PURCHASE_CHARGES)
            .short("p")
            .takes_value(true)
            .required(true)
            .help("the purchase charges"),
        Arg::with_name(ARG_ANNUAL_CHARGES)
            .long(ARG_ANNUAL_CHARGES)
            .short("a")
            .takes_value(true)
            .required(true)
            .help("the annual charges"),
        Arg::with_name(ARG_HOME_APPRECIATION)
            .long(ARG_HOME_APPRECIATION)
            .short("e")
            .takes_value(true)
            .required(true)
            .help("the home appreciation by year"),
        Arg::with_name(ARG_HOME_RENT)
            .long(ARG_HOME_RENT)
            .short("m")
            .takes_value(true)
            .required(true)
            .help("the home rent for a month"),
        Arg::with_name(ARG_INVEST_RATE_RENT)
            .long(ARG_INVEST_RATE_RENT)
            .short("i")
            .takes_value(true)
            .required(true)
            .help("the investment interest rate"),
        Arg::with_name(ARG_YEARS)
            .long(ARG_YEARS)
            .short("y")
            .takes_value(true)
            .required(true)
            .help("the years for the purchase"),
    ]
}

/// Parse the common home arguments from the cli
///
/// # Arguments
/// *  `matches` - cli arguments matches
pub fn parse_common_home_args<'a>(matches: &ArgMatches<'a>) -> HomeInvest {
    HomeInvest::new(
        matches
            .value_of(ARG_SUPPLY)
            .unwrap()
            .parse::<u32>()
            .unwrap(),
        matches.value_of(ARG_LOAN).unwrap().parse::<u32>().unwrap(),
        matches
            .value_of(ARG_LOAN_RATE)
            .unwrap()
            .parse::<f32>()
            .unwrap()
            / 100_f32,
        matches
            .value_of(ARG_PURCHASE_CHARGES)
            .unwrap()
            .parse::<f32>()
            .unwrap()
            / 100_f32,
        matches
            .value_of(ARG_ANNUAL_CHARGES)
            .unwrap()
            .parse::<f32>()
            .unwrap()
            / 100_f32,
        matches
            .value_of(ARG_HOME_APPRECIATION)
            .unwrap()
            .parse::<f32>()
            .unwrap()
            / 100_f32,
        matches
            .value_of(ARG_HOME_RENT)
            .unwrap()
            .parse::<u32>()
            .unwrap(),
        matches
            .value_of(ARG_INVEST_RATE_RENT)
            .unwrap()
            .parse::<f32>()
            .unwrap()
            / 100_f32,
        matches.value_of(ARG_YEARS).unwrap().parse::<u8>().unwrap(),
    )
}
