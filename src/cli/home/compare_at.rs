use super::{common_home_args, parse_common_home_args};
use clap::{App, Arg, ArgMatches, SubCommand};
use comparators;

pub const SUB_HOME_COMPARE_AT: &str = "compare-at";
const ARG_PERIOD: &str = "n-periods";

/// Returns the home compare-at sub command
pub fn home_compare_at_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(SUB_HOME_COMPARE_AT)
        .about("compute investement info for a point in time")
        .arg(
            Arg::with_name(ARG_PERIOD)
                .takes_value(true)
                .required(true)
                .index(1),
        ).args(common_home_args().as_slice())
}

/// Execute the work and print results for the compare-at sub command
///
/// # Arguments
/// * `matches` - The command matches to retrieve the parameters
pub fn execute_home_compare_at<'a>(matches: &ArgMatches<'a>) {
    let home_invest = parse_common_home_args(matches);
    let at = matches
        .value_of(ARG_PERIOD)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let (purchase, invest) = home_invest.capital_at(at);
    println!(
        "*** For a supply of {}, a loan of {} on {} years with a rate of {}%, \
         purchase charges of {}%, annual charges of {}% and an home appreciation of {}% by year",
        home_invest.supply,
        home_invest.loan,
        home_invest.years,
        home_invest.loan_rate * 100_f32,
        home_invest.purchase_charges * 100_f32,
        home_invest.annual_charges * 100_f32,
        home_invest.annual_appreciation_rate * 100_f32
    );
    println!(
        "*** Compared to a rent of {}, an investement interest rate at {}%",
        home_invest.rent,
        home_invest.invest_rate * 100_f32
    );
    let years_round = format!("{}", at / comparators::PERIODICITY as u32);
    let mut table = table!(["title", "at (periods)", "at (~years)", "value"]);
    table.add_row(row![
        "term price for home purchase",
        "NONE",
        "NONE",
        format!("{:.02}", home_invest.loan_term_price())
    ]);
    table.add_row(row![
        "capital for home purchase",
        at,
        years_round,
        format!("{:.02}", purchase)
    ]);
    table.add_row(row![
        "capital for renting",
        at,
        years_round,
        format!("{:.02}", invest)
    ]);
    table.add_row(row![
        "Difference for home purchase",
        at,
        years_round,
        format!("{:.02}", purchase - invest)
    ]);
    table.printstd();
}
