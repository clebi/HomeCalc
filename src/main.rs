extern crate clap;

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
            "loan of {} during {} years with period of {} at {}%",
            loan.capital, loan.years, loan.period, loan.interest_rate_year * 100_f32
        );
        println!("term price: {:.2}", loan.term_price());
        println!("capital at {}: {:.2}", at, loan.capital_at(at));
        println!("paid at {}: {:.2}", at, loan.paid(at));
        println!("interest at {}: {:.2}", at, loan.interest_at(at));
    }
}
