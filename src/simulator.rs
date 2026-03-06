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