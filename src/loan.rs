/// A Loan
pub struct Loan {
    years: u8,
    period: u8,
    interest_rate_year: f32,
    capital: u32,
    term_price: f64,
}

impl Loan {
    /// Returns a load given all its parameters
    ///
    /// # Aguments
    ///
    /// * `years` - The number oy years for the loan
    /// * `period` - perodicity of the loan
    /// * `interest_by_year` - interest rate by year of the loan
    /// * `capital` - capital of the loan
    ///
    /// # Example
    ///
    /// ```
    /// use load::Loan;
    /// let loan_8y_50k = Loan::new(8, 12, 4.5, 50_000);
    /// ```
    pub fn new(years: u8, period: u8, interest_rate_year: f32, capital: u32) -> Loan {
        let term_price = Loan::compute_term_price(capital, interest_rate_year, years, period);
        Loan {
            years,
            period,
            interest_rate_year,
            capital,
            term_price,
        }
    }

    fn compute_term_price(capital: u32, interest_rate_year: f32, years: u8, period: u8) -> f64 {
        let inretest_rate_term = interest_rate_year as f64 / period as f64;

        let term_price = (capital as f64) * inretest_rate_term
            / (1_f64 - (1_f64 + inretest_rate_term).powf(years as f64 * period as f64 * -1_f64));
        (term_price * 100_f64).round() / 100_f64
    }

    /// Return the term price of the loan
    pub fn term_price(&self) -> f64 {
        self.term_price
    }

    /// Return the capital paid at a moment of the loan
    ///
    /// # Arguments
    /// * `n_period` - number of period
    ///
    /// # Example
    /// ```
    /// // We have a loan with perodicity of 12 terms in a year
    /// // Get the capital paid at 2 years
    /// let capital_at_2y = loan.capital_at(24);
    /// ```
    pub fn capital_at(&self, n_period: i32) -> f64 {
        let inretest_rate_term = self.interest_rate_year as f64 / self.period as f64;
        let capital_n = ((1_f64 + inretest_rate_term).powf(n_period as f64) - 1_f64)
            / ((1_f64 + inretest_rate_term).powf(self.years as f64 * self.period as f64) - 1_f64);
        (capital_n * self.capital as f64 * 100_f64).round() / 100_f64
    }

    /// Return the amount paid at a moment of the loan
    ///
    /// # Arguments
    /// * `n_period` - number of period
    ///
    /// # Example
    /// ```
    /// // We have a loan with perodicity of 12 terms in a year
    /// let paid_at_1y = loan.paid_at(12);
    /// ```
    pub fn paid(&self, n_period: i32) -> f64 {
        (self.term_price * n_period as f64 * 100_f64).round() / 100_f64
    }

    /// Return the interest paid at a moment of the loan
    ///
    /// # Arguments
    /// * `n_period` - number of period
    ///
    /// # Example
    /// ```
    /// // We have a loan with perodicity of 12 terms in a year
    /// let interest_at_4y = loan.interest_at(48);
    /// ```
    pub fn interest_at(&self, n_period: i32) -> f64 {
        ((self.paid(n_period) - self.capital_at(n_period)) * 100_f64).round() / 100_f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term_price() {
        let loan = Loan::new(20, 12, 0.029_f32, 90_000);
        assert_eq!(494.64, loan.term_price());
    }

    #[test]
    fn test_capital_at() {
        let loan = Loan::new(20, 12, 0.029_f32, 90_000);
        assert_eq!(1954.13, loan.capital_at(7));
        assert_eq!(2801.78, loan.capital_at(10));
        assert_eq!(34874.26, loan.capital_at(110));
        assert_eq!(72965.12, loan.capital_at(204));
    }

    #[test]
    fn test_paid() {
        let loan = Loan::new(20, 12, 0.029_f32, 90_000);
        assert_eq!(4451.76, loan.paid(9));
        assert_eq!(15333.84, loan.paid(31));
        assert_eq!(83099.52, loan.paid(168));
    }

    #[test]
    fn test_interest_at() {
        let loan = Loan::new(20, 12, 0.029_f32, 90_000);
        assert_eq!(2355.34, loan.interest_at(11));
        assert_eq!(22487.93, loan.interest_at(134));
        assert_eq!(27899.18, loan.interest_at(203));
    }
}
