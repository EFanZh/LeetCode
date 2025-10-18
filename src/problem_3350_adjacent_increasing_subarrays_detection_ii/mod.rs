pub mod greedy;

pub trait Solution {
    fn max_increasing_subarrays(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 5, 7, 8, 9, 2, 3, 4, 3, 1] as &[_], 3),
            (&[1, 2, 3, 4, 4, 4, 4, 5, 6, 7], 2),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_increasing_subarrays(nums.to_vec()), expected);
        }
    }
}
