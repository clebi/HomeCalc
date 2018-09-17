extern crate clap;
#[macro_use]
extern crate prettytable;

/// The CLI module
mod cli;
/// The loan module
mod loan;

use clap::App;
use cli::loan::{execute_loan_sub_command, loan_sub_command, SUB_LOAN};

fn main() {
    let app_matches = App::new("HomeCalc")
        .version("1.0")
        .author("Clément Bizeau")
        .about("Various computations about homes")
        .subcommand(loan_sub_command())
        .get_matches();

    match app_matches.subcommand() {
        (SUB_LOAN, Some(loan_matches)) => execute_loan_sub_command(loan_matches),
        _ => println!("*** No command found"),
    }
}
