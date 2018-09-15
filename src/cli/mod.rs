use clap::{App, Arg, SubCommand};

pub const SUB_LOAN: &str = "loan";
pub const SUB_LOAN_INFO_AT: &str = "info-at";

fn loan_info_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(SUB_LOAN_INFO_AT)
        .about("compute loans info for a point in time")
        .arg(Arg::with_name("n-period").takes_value(true).required(true).index(1))
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
        )
}

pub fn loan_sub_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(SUB_LOAN).subcommand(loan_info_subcommand())
}
