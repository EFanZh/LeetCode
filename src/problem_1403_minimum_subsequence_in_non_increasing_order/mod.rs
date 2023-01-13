pub mod greedy_binary_heap;

pub trait Solution {
    fn min_subsequence(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[4, 3, 10, 9, 8] as &[_], &[10, 9] as &[_]),
            (&[4, 4, 7, 6, 7], &[7, 7, 6]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_subsequence(nums.to_vec()), expected);
        }
    }
}
