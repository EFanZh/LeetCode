pub mod iterative;

pub trait Solution {
    fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[3, 50], [7, 10], [12, 25]] as &[_], 10), 2.65),
            ((&[[1, 0], [4, 25], [5, 50]], 2), 0.25),
            ((&[[2, 50]], 0), 0.0),
        ];

        for ((brackets, income), expected) in test_cases {
            approx::assert_ulps_eq!(
                S::calculate_tax(brackets.iter().map(Vec::from).collect(), income),
                expected,
            );
        }
    }
}
