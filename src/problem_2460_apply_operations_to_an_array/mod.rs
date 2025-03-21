pub mod iterative;

pub trait Solution {
    fn apply_operations(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 2, 1, 1, 0] as &[_], &[1, 4, 2, 0, 0, 0] as &[_]),
            (&[0, 1], &[1, 0]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::apply_operations(nums.to_vec()), expected);
        }
    }
}
