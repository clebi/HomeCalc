extern crate clap;
#[macro_use]
extern crate prettytable;

/// The CLI module
mod cli;
/// The comparators module
mod comparators;
/// The investment module
mod investment;
/// The loan module
mod loan;

use clap::App;
use cli::invest::{execute_invest_sub_command, invest_sub_commands, SUB_INVEST};
use cli::loan::{execute_loan_sub_command, loan_sub_command, SUB_LOAN};
use cli::home::{execute_home_sub_command, home_sub_commands, SUB_HOME};

fn main() {
    let app_matches = App::new("HomeCalc")
        .version("0.0.1")
        .author("Clément Bizeau")
        .about("Various computations about homes")
        .subcommands(loan_sub_command())
        .subcommands(invest_sub_commands())
        .subcommands(home_sub_commands())
        .get_matches();

    match app_matches.subcommand() {
        (SUB_LOAN, Some(loan_matches)) => execute_loan_sub_command(loan_matches),
        (SUB_INVEST, Some(invest_matches)) => execute_invest_sub_command(invest_matches),
        (SUB_HOME, Some(home_matches)) => execute_home_sub_command(home_matches),
        _ => println!("*** No command found"),
    }
}
