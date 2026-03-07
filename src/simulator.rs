pub struct SimulatorParameters {
    pub mortgage : f64,
    pub annual_interest_rate : f64,
    pub term_length_years : u32,
    pub pay_down_principal_first: bool
}

pub struct SimulatorResults{
    pub monthly_payments : f64
}

pub struct MonthlyRecord {
    pub amount_paid_to_principal: f64,
    pub amount_paid_to_interest: f64,
    pub remaining_principal: f64
}

const MAX_STEP_SIZE:f64 = 5.0;

fn compute_monthly_interest_rate(annual_interest_rate: f64, canadian_style: bool) -> f64 {
    if canadian_style {
        (1.0 + (annual_interest_rate / 100.0 / 2.0)).powf(1.0/6.0) - 1.0
    } else {
        annual_interest_rate / 100.0 / 12.0
    } 
}

fn how_long_to_pay(per_month:f64, annual_interest:f64, principal:f64) -> f64 {
    let monthly_interest = compute_monthly_interest_rate(annual_interest, true);

    let mut months = 0;
    let mut remaining_principal = principal;
    loop {
        let how_much_we_pay_in_interest = monthly_interest * remaining_principal;
        if (per_month - how_much_we_pay_in_interest) < 0.0 {
            // Impossible to repay loan
            return 1000.0;
        }
        remaining_principal = remaining_principal - (per_month - how_much_we_pay_in_interest);
        if remaining_principal < 0.0 {
            return months as f64;
        }
        months = months + 1;
    }
}

pub fn compute_monthly_payments(principal:f64, annual_interest: f64, term_length_years: u32) -> f64 {
    let term_length_months = (term_length_years as f64) * 12.0;
    // Pay just the interest per month AND some small step size towards the principal
    let mut payment_guess = (principal * compute_monthly_interest_rate(annual_interest,true)) + MAX_STEP_SIZE;
    
    struct Guess {
        payment:f64,
        term_length:f64
    }

    let mut previous_guess : Option<Guess> = None;
    
    loop {
        let term_length_guess = how_long_to_pay(payment_guess, annual_interest, principal);
        if (term_length_guess - term_length_months) == 0.0 {
            return payment_guess
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
                    let delta_correction = delta_error * (1.0 / term_deriv);
                    payment_guess + delta_correction.clamp(-MAX_STEP_SIZE, MAX_STEP_SIZE)
                }
            };
            previous_guess = Some(Guess{
                payment:payment_guess,
                term_length:term_length_guess
            });
            payment_guess = next_payment_guess;
        }
    }
}

pub fn run_simulator(params:SimulatorParameters) -> SimulatorResults {
    SimulatorResults{
        monthly_payments : 24.0
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
            300000.0);
        assert!(how_many_months == 300.0);
    }

    #[test]
    fn compute_monthly_payments_test() {
        let monthly = compute_monthly_payments( 
            300000.0, 
             5.25, 
            25);
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
            pay_down_principal_first: false
        });
        assert!( match r.monthly_payments {
            2979.0..2981.0 => true,
            _ => false
        } );
    }
}
