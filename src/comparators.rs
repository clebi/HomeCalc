use investment::Investment;
use loan::Loan;

/// The default periodicity for an home purchase
pub const PERIODICITY: u8 = 12;

/// A home and invest comparator
pub struct HomeInvest {
    pub supply: u32,
    pub loan: u32,
    pub loan_rate: f32,
    pub purchase_charges: f32,
    pub annual_charges: f32,
    pub annual_appreciation_rate: f32,
    pub rent: u32,
    pub invest_rate: f32,
    pub years: u8,
}

impl HomeInvest {
    pub fn new(
        supply: u32,
        loan: u32,
        loan_rate: f32,
        purchase_charges: f32,
        annual_charges: f32,
        annual_appreciation_rate: f32,
        rent: u32,
        invest_rate: f32,
        years: u8,
    ) -> HomeInvest {
        HomeInvest {
            supply,
            loan,
            loan_rate,
            purchase_charges,
            annual_charges,
            annual_appreciation_rate,
            rent,
            invest_rate,
            years,
        }
    }

    /// Return the capital at some point for a real estate purchase and a financial investment
    ///
    /// # Arguments
    /// * `period` - the comparation point
    pub fn capital_at(&self, period: u32) -> (f64, f64) {
        let loan = Loan::new(self.years, PERIODICITY, self.loan_rate, self.loan);
        let invest = Investment::new(
            self.supply,
            PERIODICITY,
            self.invest_rate,
            loan.term_price() as u32 - self.rent,
        );
        let total_paid = self.supply + self.loan;
        let home_value = total_paid as f64 / (1_f64 + self.purchase_charges as f64);
        let loan_capital = self.supply as f64 + loan.capital_at(period)
            - (total_paid as f64 - home_value)
            - (self.annual_charges as f64 / PERIODICITY as f64) * home_value * period as f64
            + home_value
                * (self.annual_appreciation_rate as f64 / PERIODICITY as f64)
                * period as f64;
        return (loan_capital, invest.capital_at(period));
    }

    /// Returns the loan term price for the home purchase
    pub fn loan_term_price(&self) -> f64 {
        let loan = Loan::new(self.years, PERIODICITY, self.loan_rate, self.loan);
        loan.term_price()
    }
}

#[cfg(test)]
mod tests {
    extern crate float_cmp;

    use self::float_cmp::ApproxEq;
    use super::*;

    #[test]
    fn test_home_invest() {
        let home_comparator =
            HomeInvest::new(43063, 344500, 0.018, 0.125, 0.02, 0.025, 1050, 0.04, 25);
        let expected_capital_loan = 136788.22;
        let expected_capital_invest = 119565.65;
        let (capital_loan, capital_invest) = home_comparator.capital_at(120);
        assert!(
            &expected_capital_loan.approx_eq(
                &capital_loan,
                2000000000.0 * ::std::f64::EPSILON,
                2000000000
            ),
            "expected: {}, actual: {}",
            expected_capital_loan,
            capital_loan
        );
        assert!(
            &expected_capital_invest.approx_eq(
                &capital_invest,
                200000000.0 * ::std::f64::EPSILON,
                200000000
            ),
            "expected: {}, actual: {}",
            expected_capital_invest,
            capital_invest
        );
    }

}
