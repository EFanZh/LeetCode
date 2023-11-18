pub mod geometric_median;

pub trait Solution {
    fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 1], [1, 0], [1, 2], [2, 1]] as &[_], 4.0),
            (&[[1, 1], [3, 3]], f64::sqrt(8.0)),
            (&[[1, 1]], 0.0),
            (&[[27, 90], [99, 75], [99, 38]], 109.303_171_735_149_06),
        ];

        for (positions, expected) in test_cases {
            approx::assert_ulps_eq!(S::get_min_dist_sum(positions.iter().map(Vec::from).collect()), expected,);
        }
    }
}
