
/// The loan module
pub mod loan;

use loan::Loan;

fn main() {
    let loan = Loan::new(20, 12, 0.045_f32, 157_000);

    println!("term price: {:.2}", loan.term_price());
    println!("capital at 24: {:.2}", loan.capital_at(24));
    println!("paid at 24: {:.2}", loan.paid(24));
    println!("interest at 24: {:.2}", loan.interest_at(24));
}
