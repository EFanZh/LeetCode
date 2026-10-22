pub mod iterative;

pub trait Solution {
    fn min_swaps(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[37, 100] as &[_], 1), (&[22, 14, 33, 7], 0), (&[18, 43, 34, 16], 2)];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_swaps(nums.to_vec()), expected);
        }
    }
}
