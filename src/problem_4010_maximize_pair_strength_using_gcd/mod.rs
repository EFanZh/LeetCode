pub mod greedy;

pub trait Solution {
    fn max_pair_strength(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 3, 5] as &[_], 15),
            (&[4, 6, 8], 12),
            (&[3, 3], 1),
            (&[2, 10, 1], 10),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_pair_strength(nums.to_vec()), expected);
        }
    }
}
