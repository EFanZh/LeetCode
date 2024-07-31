pub mod stack;

pub trait Solution {
    fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[6, 4, 3, 2, 7, 6, 2] as &[_], &[12, 7, 6] as &[_]),
            (&[2, 2, 1, 1, 3, 3, 3], &[2, 1, 1, 3]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::replace_non_coprimes(nums.to_vec()), expected);
        }
    }
}
