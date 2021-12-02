pub mod dynamic_programming;

pub trait Solution {
    fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[allow(clippy::manual_assert)]
    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[9, 1, 2, 3, 9] as &[_], 3), 20.0),
            ((&[1, 2, 3, 4, 5, 6, 7], 4), 20.5),
        ];

        for ((nums, k), expected) in test_cases {
            approx::assert_ulps_eq!(S::largest_sum_of_averages(nums.to_vec(), k), expected);
        }
    }
}
