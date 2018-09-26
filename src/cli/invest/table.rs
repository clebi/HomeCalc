use super::{common_invest_args, parse_common_invest_args};
use clap::{App, Arg, ArgMatches, SubCommand};
use investment::Investment;
use prettytable::row::Row;

pub const SUB_INVEST_TABLE: &str = "table";
const ARG_EVERY_PERIOD: &str = "every-period";
const ARG_TO: &str = "to";

/// Returns the loan info-at sub command
pub fn invest_table_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(SUB_INVEST_TABLE)
        .about("print the amotization table for a loan")
        .arg(
            Arg::with_name(ARG_EVERY_PERIOD)
                .takes_value(true)
                .required(true)
                .index(1),
        ).arg(
            Arg::with_name(ARG_TO)
                .takes_value(true)
                .required(true)
                .index(2),
        ).args(common_invest_args().as_slice())
}

/// Execute the work and print results for the info-at sub command
///
/// # Arguments
/// * `matches` - The command matches to retrieve the paramters
pub fn execute_invest_table<'a>(matches: &ArgMatches<'a>) {
    let invest = parse_common_invest_args(matches);
    let every = matches
        .value_of(ARG_EVERY_PERIOD)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let to = matches.value_of(ARG_TO).unwrap().parse::<u32>().unwrap();

    println!("*** For an investment of {} and regular additions of {} per period at a interest rate of {}% per year\n",
            invest.capital, invest.regular_addition, invest.yield_rate * 100_f32);

    let mut invest_table = table!([
        "At (periods)",
        "At (~years)",
        "Total additions",
        "Total invest",
        "Capital",
        "interest earned",
    ]);
    invest_table.add_row(get_row(&invest, 0));
    for at in (every..to).step_by(every as usize) {
        invest_table.add_row(get_row(&invest, at));
    }
    invest_table.add_row(get_row(&invest, to));
    invest_table.printstd();
}

fn get_row(invest: &Investment, at: u32) -> Row {
    let years_round = format!("{:.1}", at as f32 / invest.periodicity as f32);
    let capital_at = invest.capital_at(at);
    let total_invest = invest.capital + invest.additions_total(at);
    row![
        at,
        years_round,
        format!("{:.2}", invest.additions_total(at)),
        format!("{:.2}", total_invest),
        format!("{:.2}", capital_at),
        format!("{:.2}", capital_at - total_invest as f64),
    ]
}
