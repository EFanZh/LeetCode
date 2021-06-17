pub mod stack;

pub trait Solution {
    fn next_greater_elements(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 1] as &[_], &[2, -1, 2] as &[_]),
            (&[1, 2, 3, 4, 3], &[2, 3, 4, -1, 4]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::next_greater_elements(nums.to_vec()), expected);
        }
    }
}
