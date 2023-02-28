use std::error::Error;
use std::fmt;

/// The Heligman-Pollard method of mortality rate estimation.

#[derive(Debug)]
pub struct HPError { problem: String }

impl fmt::Display for HPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.problem)
    }
}

impl Error for HPError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
    fn description(&self) -> &str {
        &self.problem
    }
}

fn clamp (x: f32, name: &str) -> Result<f32, HPError> {
    if 0.0 > x || x > 1.0 {
        Err(HPError { problem: format!("Value of {} out of range 0...1.", name) })
    } else {
        Ok(x)
    }
}

fn positive (x: f32, name: &str) -> Result<f32, HPError> {
    if x.is_sign_positive() {
        Ok(x)
    } else {
        Err(HPError { problem: format!("Value of {} is not positive.", name) })
    }
}

/// The parameters of an H.-P. equation.
///
/// I've given the variables appropriate names for their intended effect on the function, but I've
/// also annotated them with the traditional (bad) variable names. Why mathematicians do this, I
/// don't know.
pub fn maybe_mortality_at_age(
    age_i: i32,
    //A
    infant_mortality: f32,
    //B
    first_year_mortality: f32,
    //C
    infant_mortality_dropoff: f32,
    //D
    accident_severity: f32,
    //E
    accident_spread: f32,
    //F
    accident_midpoint: f32,
    //G
    adult_mortality: f32,
    //H
    adult_mortality_increase: f32,
) -> Result<f32, HPError> {
    let infant_mortality = clamp(infant_mortality, "infant_mortality")?;
    let first_year_mortality = clamp(first_year_mortality, "first_year_mortality")?;
    let infant_mortality_dropoff = clamp(infant_mortality_dropoff, "infant_mortality_dropoff")?;
    let accident_severity = clamp(accident_severity, "accident_severity")?;
    let accident_spread = positive(accident_spread, "accident_spread")?;
    let accident_midpoint = positive(accident_midpoint, "accident_midpoint")?;
    let adult_mortality = clamp(adult_mortality, "adult_mortality")?;
    let adult_mortality_increase = positive(adult_mortality_increase, "adult_mortality_increase")?;

    let age = age_i as f32;
    let early_exponent = (age + first_year_mortality).powf(infant_mortality_dropoff);
    let early_life = infant_mortality.powf(early_exponent);

    let accident_power = -accident_spread * (age / accident_midpoint).log10().powi(2);
    let accident_spike = accident_severity * accident_power.exp();

    let old_age = adult_mortality * adult_mortality_increase.powi(age_i);

    let combined_formula = early_life + accident_spike + old_age; // q/(1-q)

    let mortality = combined_formula / (combined_formula + 1.0);

    Ok(mortality)

}

pub fn mortality_at_age (
    age_i: i32,
    infant_mortality: f32,
    first_year_mortality: f32,
    infant_mortality_dropoff: f32,
    accident_severity: f32,
    accident_spread: f32,
    accident_midpoint: f32,
    adult_mortality: f32,
    adult_mortality_increase: f32,
) -> f32 {
    maybe_mortality_at_age(
        age_i,
        infant_mortality,
        first_year_mortality,
        infant_mortality_dropoff,
        accident_severity,
        accident_spread,
        accident_midpoint,
        adult_mortality,
        adult_mortality_increase,
    ).unwrap_or(0.0)
}

pub fn baseline_male(age_i: i32) -> f32 {
    mortality_at_age(age_i, 0.0004, 0.0192, 0.1048, 0.001, 9.0, 21.0, 0.0001, 1.1)
}

pub fn baseline_female(age_i: i32) -> f32 {
    mortality_at_age(age_i, 0.0004, 0.0192, 0.1048, 0.0004, 3.0, 19.0, 0.00002, 1.1)
}   

pub fn vary_infant_mortality(age_i: i32, infant_mortality: f32) -> f32 {
    mortality_at_age(age_i, infant_mortality, 0.0192, 0.1048, 0.001, 9.0, 21.0, 0.0001, 1.1)
}

