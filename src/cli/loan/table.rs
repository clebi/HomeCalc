use clap::{App, Arg, ArgMatches, SubCommand};
use loan::Loan;
use prettytable::row::Row;

pub const SUB_LOAN_TABLE: &str = "table";
const ARG_EVERY_PERIOD: &str = "every-period";
const ARG_YEARS: &str = "years";
const ARG_PERIODICITY: &str = "periodicity";
const ARG_INTEREST_RATE: &str = "interest-rate";
const ARG_CAPITAL: &str = "capital";

/// Returns the loan info-at sub command
pub fn loan_table_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(SUB_LOAN_TABLE)
        .about("print the amotization table for a loan")
        .arg(
            Arg::with_name(ARG_EVERY_PERIOD)
                .takes_value(true)
                .required(true)
                .index(1),
        ).arg(
            Arg::with_name(ARG_YEARS)
                .long(ARG_YEARS)
                .short("y")
                .takes_value(true)
                .required(true)
                .help("Number of years for the loan"),
        ).arg(
            Arg::with_name(ARG_PERIODICITY)
                .long(ARG_PERIODICITY)
                .short("p")
                .takes_value(true)
                .required(true)
                .help("perodicity for the loan (by year)"),
        ).arg(
            Arg::with_name(ARG_INTEREST_RATE)
                .long(ARG_INTEREST_RATE)
                .short("i")
                .takes_value(true)
                .required(true)
                .help("interest rate for the loan in percent"),
        ).arg(
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
pub fn execute_loan_table<'a>(matches: &ArgMatches<'a>) {
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
            .unwrap()
            / 100_f32,
        matches
            .value_of(ARG_CAPITAL)
            .unwrap()
            .parse::<u32>()
            .unwrap(),
    );
    let every = matches
        .value_of(ARG_EVERY_PERIOD)
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

    let mut loan_table = table!([
        "At (periods)",
        "At (~years)",
        "Ending balance",
        "Capital paid",
        "Total interest",
        "~Interest overhead ratio",
    ]);
    loan_table.add_row(get_row(&loan, 1));
    for at in (every..(loan.years as u32 * loan.period as u32)).step_by(every as usize) {
        loan_table.add_row(get_row(&loan, at));
    }
    loan_table.add_row(get_row(&loan, loan.years as u32 * loan.period as u32));
    loan_table.printstd();
}

fn get_row(loan: &Loan, at: u32) -> Row {
    let years_round = format!("{:.1}", at as f32 / loan.period as f32);
    let capital_paid = loan.capital_at(at);
    let interest_paid = loan.interest_at(at);
    row![
        at,
        years_round,
        format!("{:.2}", loan.capital as f64 - capital_paid),
        format!("{:.2}", capital_paid),
        format!("{:.2}", interest_paid),
        format!("{:.2}%", (interest_paid / capital_paid) * 100_f64),
    ]
}
