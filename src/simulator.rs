use core::f64;

pub enum CompoundingInterestStyle{
    American,
    Canadian,
    Fair
}

fn compute_monthly_interest_rate(annual_interest_rate: f64, interest_style: &CompoundingInterestStyle) -> f64 {
    match interest_style {
        CompoundingInterestStyle::Canadian => (1.0 + (annual_interest_rate / 100.0 / 2.0)).powf(1.0/6.0) - 1.0,
        CompoundingInterestStyle::American => annual_interest_rate / 100.0 / 12.0,
        CompoundingInterestStyle::Fair => (1.0 + (annual_interest_rate / 100.0 / 2.0)).powf(1.0/12.0) - 1.0,
    }
}

fn how_long_to_pay(per_month:f64, annual_interest:f64, principal:f64, compounding_style: &CompoundingInterestStyle) -> f64 {
    let monthly_interest = compute_monthly_interest_rate(annual_interest, compounding_style);

    let mut months = 0;
    let mut remaining_principal = principal;
    loop {
        let how_much_we_pay_in_interest = monthly_interest * remaining_principal;
        if (per_month - how_much_we_pay_in_interest) < 0.0 {
            // Impossible to repay loan
            return f64::INFINITY;
        }
        remaining_principal = remaining_principal - (per_month - how_much_we_pay_in_interest);
        if remaining_principal < 0.0 {
            return months as f64;
        }
        months = months + 1;
    }
}


fn compute_monthly_payments(principal:f64, annual_interest: f64, term_length_years: u32, compounding_style: &CompoundingInterestStyle) -> f64 {
    
    const NARROW_PHASE_STEP_SIZE:f64 = 0.10;
    let max_step_size:f64 = (principal / 1e6  ).max(0.01);
    
    let term_length_months = (term_length_years as f64) * 12.0;
    let mut payment_guess = (principal * compute_monthly_interest_rate(annual_interest, compounding_style)) + max_step_size;
    
    struct Guess {
        payment:f64,
        term_length:f64
    }

    let mut previous_guess : Option<Guess> = None;
    enum CurrentPhase{
        Broad,
        Narrow
    }
    let mut phase = CurrentPhase::Broad;
    
    loop {
        let term_length_guess = how_long_to_pay(payment_guess, annual_interest, principal, compounding_style);
        match phase {
            CurrentPhase::Broad => {
                if (term_length_guess - term_length_months) == 0.0 {
                    phase = CurrentPhase::Narrow;
                } else {
                    let next_payment_guess = match previous_guess {
                        None => {
                            if term_length_guess > term_length_months {
                                payment_guess * 1.05
                            } else {
                                payment_guess * 0.95
                            }
                        }
                        Some(p) => {
                            let delta_payment = payment_guess - p.payment;
                            let delta_term = (term_length_guess - p.term_length) as f64;
                            let term_deriv = delta_term / delta_payment;
                            let delta_error = (term_length_months - term_length_guess) as f64;
                            let delta_correction = if term_deriv != 0.0 {
                                delta_error * (1.0 / term_deriv)
                            } else {
                                if term_length_guess > term_length_months {
                                    max_step_size
                                } else {
                                    -max_step_size
                                }
                            };
                            payment_guess + delta_correction.clamp(-max_step_size, max_step_size)
                        }
                    };
                    previous_guess = Some(Guess{
                        payment:payment_guess,
                        term_length:term_length_guess
                    });
                    payment_guess = next_payment_guess;
                    if payment_guess < NARROW_PHASE_STEP_SIZE {
                        return NARROW_PHASE_STEP_SIZE // honestly who cares
                    }
                }
            }
            CurrentPhase::Narrow => {
                if (term_length_guess - term_length_months) == 0.0 {
                    payment_guess = payment_guess + NARROW_PHASE_STEP_SIZE
                } else {
                    return payment_guess - NARROW_PHASE_STEP_SIZE
                }
            }
        }
    }
}

pub enum SimulatorError{
    InvalidParameter
}


pub struct SimulatorParameters {
    mortgage : f64,
    annual_interest_rate : f64,
    term_length_years : u32,
    compounding_style: CompoundingInterestStyle
}

impl SimulatorParameters {
    pub fn new(
        mortgage : f64,
        annual_interest_rate : f64,
        term_length_years : u32,
        compounding_style: CompoundingInterestStyle
    ) -> Result<Self, SimulatorError> {
        if mortgage < 100.0 || mortgage > 10.0e9 {
            return Err(SimulatorError::InvalidParameter);
        }
        if term_length_years <= 0 || term_length_years > 50 {
            return Err(SimulatorError::InvalidParameter);
        }
        if annual_interest_rate < 0.0 || annual_interest_rate > 50.0 {
            return Err(SimulatorError::InvalidParameter);
        }
        Ok(Self{
            mortgage,
            annual_interest_rate,
            term_length_years,
            compounding_style
        })
    }
}

pub struct SimulatorResults{
    pub monthly_payment : f64,
    pub timeline : Vec<MonthlyRecord>
}

#[derive(Debug)]
pub struct MonthlyRecord {
    pub amount_paid_to_principal: f64,
    pub amount_paid_to_interest: f64,
    pub remaining_principal: f64
}

pub fn run_simulator(params:SimulatorParameters) -> SimulatorResults {
    let monthly_payment = compute_monthly_payments(params.mortgage,params.annual_interest_rate,params.term_length_years,&params.compounding_style);
    let monthly_interest = compute_monthly_interest_rate(params.annual_interest_rate, &params.compounding_style);
    
    let mut timeline = Vec::<MonthlyRecord>::new();

    let mut remaining_principal = params.mortgage;
    let mut month = 1;
    loop {
        let amount_paid_to_interest = monthly_interest * remaining_principal;
        let amount_paid_to_principal = (monthly_payment - amount_paid_to_interest).min(remaining_principal);
        remaining_principal = (remaining_principal - amount_paid_to_principal).max(0.0);
        timeline.push(MonthlyRecord { 
            amount_paid_to_principal, 
            amount_paid_to_interest, 
            remaining_principal 
        } );
        if remaining_principal == 0.0 {
            break;
        } else {
            month = month + 1;
        }        
    }
    SimulatorResults{
        monthly_payment: monthly_payment,
        timeline
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_term_length_test() {
        let how_many_months = how_long_to_pay( 
            1787.75,
            5.25, 
            300000.0,
            &CompoundingInterestStyle::Canadian);
        assert!(how_many_months == 300.0);
    }

    #[test]
    fn compute_monthly_payments_test() {
        let monthly = compute_monthly_payments( 
            300000.0, 
            5.25, 
            25,
            &CompoundingInterestStyle::Canadian);
        assert!( match monthly {
            1787.0..1789.0 => true,
            _ => false
        } );
    }

    #[test]
    fn simple_payment_test() {
        let r = run_simulator(SimulatorParameters { 
            mortgage: 500000.0, 
            annual_interest_rate: 5.25, 
            term_length_years: 25, 
            compounding_style: CompoundingInterestStyle::Canadian
        });
        assert!( match r.monthly_payment {
            2979.0..2981.0 => true,
            _ => false
        } );
    }

    #[test]
    fn crazy_small_mortgage_test() {
        let r = run_simulator(SimulatorParameters {
            mortgage: 25.0, 
            annual_interest_rate: 5.1, 
            term_length_years: 10, 
            compounding_style: CompoundingInterestStyle::Canadian
        });
        assert!( match r.monthly_payment {
            0.25..0.27 => true,
            _ => false
        } );
    }
}
