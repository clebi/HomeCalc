extern crate clap;
#[macro_use]
extern crate prettytable;

/// The CLI module
pub mod cli;
/// The loan module
pub mod loan;

use clap::App;
use cli::{loan_sub_command, SUB_LOAN, SUB_LOAN_INFO_AT};
use loan::Loan;

fn main() {
    let app_matches = App::new("HomeCalc")
        .version("1.0")
        .author("Clément Bizeau")
        .about("Various computations about homes")
        .subcommand(loan_sub_command())
        .get_matches();

    match app_matches.subcommand() {
        (SUB_LOAN, Some(loan_matches)) => match loan_matches.subcommand() {
            (SUB_LOAN_INFO_AT, Some(info_at_matches)) => {
                let loan = Loan::new(
                    info_at_matches
                        .value_of("years")
                        .unwrap()
                        .parse::<u8>()
                        .unwrap(),
                    info_at_matches
                        .value_of("periodicity")
                        .unwrap()
                        .parse::<u8>()
                        .unwrap(),
                    info_at_matches
                        .value_of("interest-rate")
                        .unwrap()
                        .parse::<f32>()
                        .unwrap() / 100_f32,
                    info_at_matches
                        .value_of("capital")
                        .unwrap()
                        .parse::<u32>()
                        .unwrap(),
                );
                let at = info_at_matches
                    .value_of("n-period")
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
            _ => println!("No command found"),
        },
        _ => println!("No command found"),
    }
}
