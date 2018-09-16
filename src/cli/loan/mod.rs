use clap::{App, SubCommand};

/// The info module whoch contains the subcommand for info-at sub command
pub mod info;

use self::info::loan_info_subcommand;

/// The loan sub command string
pub const SUB_LOAN: &str = "loan";

/// Create the sub command loan
pub fn loan_sub_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(SUB_LOAN).subcommand(loan_info_subcommand())
}
