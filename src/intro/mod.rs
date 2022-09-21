mod bayes;
mod condtionals_prop;
mod distributions;
mod sequence;

/// Equally Likely Outcomes
/// a: Number of desired outcomes
/// b: Total number of possible outcomes
/// 
/// Example: 
///
/// /// Two six-sided dice. What is the propability sum of dice are 2?
/// One outcome where this is true (1,1)
/// Total Number of outcomes 6^2 = 36
/// ```
/// assert_eq(equal_likely_outcomes(1, 36), (1/36)f32)
/// ```
pub fn equal_likely_outcomes(a: &f32, b: &f32) -> f32 {
    a/b
}

pub fn roll_dice(n_sides: i32, n_dice: i32) -> Vec<i32> {
    unimplemented!()
}
