pub mod greedy;

pub trait Solution {
    fn min_distinct_freq_pair(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 1, 2, 2, 3, 4] as &[_], &[1, 3] as &[_]),
            (&[1, 5], &[-1, -1]),
            (&[7], &[-1, -1]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_distinct_freq_pair(nums.to_vec()), expected);
        }
    }
}
