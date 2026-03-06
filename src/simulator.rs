pub struct SimulatorParameters {
    pub mortgage : f64,
    pub annual_interest_rate : f64,
    pub term_length_years : u32,
    pub pay_down_principal_first: bool
}

pub struct SimulatorResults{
    pub monthly_payments : f64
}

pub fn run_simulator(params:SimulatorParameters) -> SimulatorResults {
    SimulatorResults { monthly_payments: params.mortgage / (params.term_length_years as f64 * 12.0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_payment_test() {
        let r = run_simulator(SimulatorParameters { 
            mortgage: 500000.0, 
            annual_interest_rate: 5.2, 
            term_length_years: 35, 
            pay_down_principal_first: false
        });
        assert!( match r.monthly_payments {
            1190.45..1190.48 => true,
            _ => false
        } );
    }
}
