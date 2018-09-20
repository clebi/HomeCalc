use super::{common_loan_args, parse_common_loan_args};
use clap::{App, Arg, ArgMatches, SubCommand};
use loan::Loan;
use prettytable::row::Row;

pub const SUB_LOAN_TABLE: &str = "table";
const ARG_EVERY_PERIOD: &str = "every-period";

/// Returns the loan info-at sub command
pub fn loan_table_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(SUB_LOAN_TABLE)
        .about("print the amotization table for a loan")
        .arg(
            Arg::with_name(ARG_EVERY_PERIOD)
                .takes_value(true)
                .required(true)
                .index(1),
        ).args(common_loan_args().as_slice())
}

/// Execute the work and print results for the info-at sub command
///
/// # Arguments
/// * `matches` - The command matches to retrieve the paramters
pub fn execute_loan_table<'a>(matches: &ArgMatches<'a>) {
    let loan = parse_common_loan_args(matches);
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
