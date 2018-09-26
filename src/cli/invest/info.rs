use super::{common_invest_args, parse_common_invest_args};
use clap::{App, Arg, ArgMatches, SubCommand};

pub const SUB_INVEST_INFO_AT: &str = "info-at";
const ARG_PERIOD: &str = "n-periods";

/// Returns the loan info-at sub command
pub fn invest_info_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(SUB_INVEST_INFO_AT)
        .about("compute investement info for a point in time")
        .arg(
            Arg::with_name(ARG_PERIOD)
                .takes_value(true)
                .required(true)
                .index(1),
        ).args(common_invest_args().as_slice())
}

/// Execute the work and print results for the info-at sub command
///
/// # Arguments
/// * `matches` - The command matches to retrieve the paramters
pub fn execute_invest_info_at<'a>(matches: &ArgMatches<'a>) {
    let invest = parse_common_invest_args(matches);
    let at = matches
        .value_of(ARG_PERIOD)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let years_round = format!("{:.1}", at as f32 / invest.periodicity as f32);
    let total_invest = invest.capital + invest.additions_total(at);
    let capital = invest.capital_at(at);
    println!("*** For an investment of {} and regular additions of {} per period at a interest rate of {} per year\n",
            invest.capital, invest.regular_addition, invest.yield_rate);
    let mut invest_table = table!(["title", "at (periods)", "at (~years)", "value"]);
    invest_table.add_row(row![
        "total additions",
        "NONE",
        "NONE",
        format!("{:.02}", invest.additions_total(at))
    ]);
    invest_table.add_row(row![
        "total investment",
        "NONE",
        "NONE",
        format!("{:.02}", total_invest)
    ]);
    invest_table.add_row(row![
        "capital",
        at,
        years_round,
        format!("{:.02}", invest.capital_at(at))
    ]);
    invest_table.add_row(row![
        "interest earned",
        at,
        years_round,
        format!("{:.02}", capital - total_invest as f64)
    ]);
    invest_table.printstd();
}
