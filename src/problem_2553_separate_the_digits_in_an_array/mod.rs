pub mod iterative;

pub trait Solution {
    fn separate_digits(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[13, 25, 83, 77] as &[_], &[1, 3, 2, 5, 8, 3, 7, 7] as &[_]),
            (&[7, 1, 3, 9], &[7, 1, 3, 9]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::separate_digits(nums.to_vec()), expected);
        }
    }
}
