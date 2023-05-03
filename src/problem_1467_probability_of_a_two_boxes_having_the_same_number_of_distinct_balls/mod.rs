pub mod dynamic_programming;

pub trait Solution {
    fn get_probability(balls: Vec<i32>) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 1] as &[_], 1.0),
            (&[2, 1, 1], 2.0 / 3.0),
            (&[1, 2, 1, 2], 0.6),
            (&[6, 6, 6, 6, 6, 6], 8_197_319_400.0 / 9_075_135_300.0),
        ];

        for (balls, expected) in test_cases {
            approx::assert_ulps_eq!(S::get_probability(balls.to_vec()), expected);
        }
    }
}
