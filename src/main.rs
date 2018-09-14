extern crate clap;
#[macro_use]
extern crate prettytable;

/// The loan module
pub mod loan;

use clap::{App, Arg, SubCommand};
use loan::Loan;

fn main() {
    let matches = App::new("HomeCalc")
        .version("1.0")
        .author("Clément Bizeau")
        .about("Various computations about homes")
        .subcommand(
            SubCommand::with_name("loan")
                .about("compute loans")
                .arg(
                    Arg::with_name("info-at")
                        .long("info-at")
                        .short("a")
                        .takes_value(true)
                        .required(true)
                        .help("Display basic inforation at n period"),
                )
                .arg(
                    Arg::with_name("years")
                        .long("years")
                        .short("y")
                        .takes_value(true)
                        .required(true)
                        .help("Number of years for the loan"),
                )
                .arg(
                    Arg::with_name("periodicity")
                        .long("periodicity")
                        .short("p")
                        .takes_value(true)
                        .required(true)
                        .help("perodicity for the loan (by year)"),
                )
                .arg(
                    Arg::with_name("interest-rate")
                        .long("interest-rate")
                        .short("i")
                        .takes_value(true)
                        .required(true)
                        .help("interest rate for the loan in percent"),
                )
                .arg(
                    Arg::with_name("capital")
                        .long("capital")
                        .short("c")
                        .takes_value(true)
                        .required(true)
                        .help("capital to borrow"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("loan") {
        let loan = Loan::new(
            matches.value_of("years").unwrap().parse::<u8>().unwrap(),
            matches
                .value_of("periodicity")
                .unwrap()
                .parse::<u8>()
                .unwrap(),
            matches
                .value_of("interest-rate")
                .unwrap()
                .parse::<f32>()
                .unwrap() / 100_f32,
            matches.value_of("capital").unwrap().parse::<u32>().unwrap(),
        );
        let at = matches.value_of("info-at").unwrap().parse::<u32>().unwrap();

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
}
