use clap::{App, Arg, ArgMatches, SubCommand};
use loan::Loan;

pub const SUB_LOAN_INFO_AT: &str = "info-at";
const ARG_N_PERIOD: &str = "n-period";
const ARG_YEARS: &str = "years";
const ARG_PERIODICITY: &str = "periodicity";
const ARG_INTEREST_RATE: &str = "interest-rate";
const ARG_CAPITAL: &str = "capital";

/// Create the loan info-at sub command
pub fn loan_info_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(SUB_LOAN_INFO_AT)
        .about("compute loans info for a point in time")
        .arg(
            Arg::with_name(ARG_N_PERIOD)
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name(ARG_YEARS)
                .long(ARG_YEARS)
                .short("y")
                .takes_value(true)
                .required(true)
                .help("Number of years for the loan"),
        )
        .arg(
            Arg::with_name(ARG_PERIODICITY)
                .long(ARG_PERIODICITY)
                .short("p")
                .takes_value(true)
                .required(true)
                .help("perodicity for the loan (by year)"),
        )
        .arg(
            Arg::with_name(ARG_INTEREST_RATE)
                .long(ARG_INTEREST_RATE)
                .short("i")
                .takes_value(true)
                .required(true)
                .help("interest rate for the loan in percent"),
        )
        .arg(
            Arg::with_name(ARG_CAPITAL)
                .long(ARG_CAPITAL)
                .short("c")
                .takes_value(true)
                .required(true)
                .help("capital to borrow"),
        )
}

/// Execute the work and print results for the info-at sub command
///
/// # Arguments
/// * `matches` - The command matches to retrieve the paramters
pub fn execute_loan_info_at<'a>(matches: &ArgMatches<'a>) {
    let loan = Loan::new(
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
            .unwrap() / 100_f32,
        matches
            .value_of(ARG_CAPITAL)
            .unwrap()
            .parse::<u32>()
            .unwrap(),
    );
    let at = matches
        .value_of(ARG_N_PERIOD)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    println!(
        "*** Information for a loan of {} during {} years with period of {} at {}% ***\n",
        loan.capital,
        loan.years,
        loan.period,
        loan.interest_rate_year * 100_f32
    );
    let years_round = format!("{:.1}", at as f32 / loan.period as f32);
    let mut loan_table = table!(["title", "at (periods)", "at (~years)", "value"]);
    loan_table.add_row(row!["term price", "NONE", "NONE", loan.term_price()]);
    loan_table.add_row(row!["capital paid", at, years_round, loan.capital_at(at)]);
    loan_table.add_row(row!["paid", at, years_round, loan.paid(at)]);
    loan_table.add_row(row!["interest paid", at, years_round, loan.interest_at(at)]);
    loan_table.printstd();
}
