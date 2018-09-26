/// An investment
pub struct Investment {
    pub capital: u32,
    pub periodicity: u8,
    pub yield_rate: f32,
    pub regular_addition: u32,
}

impl Investment {
    /// Returns an investment given all its parameters
    ///
    /// # Aguments
    ///
    /// * `capital` - capital for the investment
    /// * `periodicity` - perodicity
    /// * `yield_rate` - yield rate by year
    ///
    /// # Example
    ///
    /// ```
    /// use investment::Investment;
    /// let invest_10k_1p_4percent = Investment::new(10_000, 1, 4_f32, 0);
    /// ```
    pub fn new(
        capital: u32,
        periodicity: u8,
        yield_rate: f32,
        regular_addition: u32,
    ) -> Investment {
        Investment {
            capital,
            periodicity,
            yield_rate,
            regular_addition,
        }
    }

    fn yield_rate_period(&self) -> f32 {
        self.yield_rate / self.periodicity as f32
    }

    fn capital_principal(&self, n_period: u32) -> f64 {
        self.capital as f64 * (1_f64 + self.yield_rate_period() as f64).powf(n_period as f64)
    }

    fn capital_additions(&self, n_period: u32) -> f64 {
        self.regular_addition as f64
            * (((1_f64 + self.yield_rate_period() as f64).powf(n_period as f64) - 1_f64)
                / self.yield_rate_period() as f64)
    }

    /// Return the capital at a moment for an investment
    ///
    /// # Arguments
    /// * `n_period` - number of period
    ///
    /// # Example
    /// ```
    /// // We have an 10k investment with perodicity of 12 and interest rate of 4%
    /// // Get the capital at 2 years with a regular addition of 100 per period
    /// let invest_10k_1p_4percent_100a = Investment::new(10_000, 12, 4_f32, 100);
    /// let capital_at_2y = loan.capital_at(24);
    /// ```
    pub fn capital_at(&self, n_period: u32) -> f64 {
        self.capital_principal(n_period) + self.capital_additions(n_period)
    }

    /// total of regular additions for a number of periods
    ///
    /// # Arguments
    /// * `n_period` - number of period
    pub fn additions_total(&self, n_period: u32) -> u32 {
        self.regular_addition * n_period
    }
}

#[cfg(test)]
mod tests {
    extern crate float_cmp;

    use super::*;
    use investment::tests::float_cmp::ApproxEq;

    #[test]
    fn test_yield_rate_period() {
        let expected = 0.005_f32;
        let invest = Investment::new(10000, 12, 0.06, 0);
        assert!(
            &expected.approx_eq(&invest.yield_rate_period(), 2.0 * ::std::f32::EPSILON, 2),
            "expected: {}, actual: {}",
            expected,
            invest.yield_rate_period()
        );
    }

    #[test]
    fn test_capital_at() {
        let expected = 43951.93_f64;
        let invest = Investment::new(25000, 12, 0.04, 150);
        assert!(
            &expected.approx_eq(&invest.capital_at(72), 2.0 * ::std::f64::EPSILON, 200000000),
            "expected: {}, actual: {}",
            expected,
            invest.capital_at(72)
        );
    }

    #[test]
    fn test_total_additions() {
        let invest = Investment::new(25000, 12, 0.04, 234);
        assert_eq!(19656, invest.additions_total(84));
    }
}
