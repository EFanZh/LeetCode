pub mod iterative;

pub trait Solution {
    fn minimum_average(nums: Vec<i32>) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[7, 8, 3, 4, 15, 13, 4, 1] as &[_], 5.5),
            (&[1, 9, 8, 3, 10, 5], 5.5),
            (&[1, 2, 3, 7, 8, 9], 5.0),
        ];

        for (nums, expected) in test_cases {
            approx::assert_ulps_eq!(S::minimum_average(nums.to_vec()), expected);
        }
    }
}
