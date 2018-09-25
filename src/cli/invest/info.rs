use clap::{App, Arg, ArgMatches, SubCommand};
use investment::Investment;

pub const SUB_INVEST_INFO_AT: &str = "info-at";
const ARG_PERIOD: &str = "n-periods";
const ARG_PERIODICITY: &str = "period";
const ARG_YIELD_RATE: &str = "yield";
const ARG_CAPITAL: &str = "capital";
const ARG_REGULAR_ADDITION: &str = "addition";

/// Returns the loan info-at sub command
pub fn invest_info_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(SUB_INVEST_INFO_AT)
        .about("compute investement info for a point in time")
        .arg(
            Arg::with_name(ARG_PERIOD)
                .takes_value(true)
                .required(true)
                .index(1),
        ).arg(
            Arg::with_name(ARG_PERIODICITY)
                .long(ARG_PERIODICITY)
                .short("p")
                .takes_value(true)
                .required(true)
                .help("perodicity for the loan (by year)"),
        ).arg(
            Arg::with_name(ARG_YIELD_RATE)
                .long(ARG_YIELD_RATE)
                .short("r")
                .takes_value(true)
                .required(true)
                .help("perodicity for the loan (by year)"),
        ).arg(
            Arg::with_name(ARG_CAPITAL)
                .long(ARG_CAPITAL)
                .short("c")
                .takes_value(true)
                .required(true)
                .help("capital to borrow"),
        ).arg(
            Arg::with_name(ARG_REGULAR_ADDITION)
                .long(ARG_REGULAR_ADDITION)
                .short("a")
                .takes_value(true)
                .required(false)
                .default_value("0")
                .help("regular addition"),
        )
}

/// Execute the work and print results for the info-at sub command
///
/// # Arguments
/// * `matches` - The command matches to retrieve the paramters
pub fn execute_invest_info_at<'a>(matches: &ArgMatches<'a>) {
    let invest = Investment::new(
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
    );
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
