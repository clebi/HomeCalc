extern crate clap;
#[macro_use]
extern crate prettytable;

/// The CLI module
pub mod cli;
/// The loan module
pub mod loan;

use clap::App;
use cli::loan::{SUB_LOAN, loan_sub_command};
use cli::loan::info::{SUB_LOAN_INFO_AT, execute_loan_info_at};

fn main() {
    let app_matches = App::new("HomeCalc")
        .version("1.0")
        .author("Clément Bizeau")
        .about("Various computations about homes")
        .subcommand(loan_sub_command())
        .get_matches();

    match app_matches.subcommand() {
        (SUB_LOAN, Some(loan_matches)) => match loan_matches.subcommand() {
            (SUB_LOAN_INFO_AT, Some(info_at_matches)) => execute_loan_info_at(info_at_matches),
            _ => println!("*** No command found"),
        },
        _ => println!("*** No command found"),
    }
}
