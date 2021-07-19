pub mod dynamic_programming;

pub trait Solution {
    fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 2, 0, 0), 0.0625), ((1, 0, 0, 0), 1.0), ((3, 1, 1, 2), 0.25)];

        for ((n, k, row, column), expected) in test_cases {
            approx::assert_ulps_eq!(S::knight_probability(n, k, row, column), expected);
        }
    }
}
