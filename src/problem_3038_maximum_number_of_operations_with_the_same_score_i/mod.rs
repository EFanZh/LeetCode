pub mod iterative;

pub trait Solution {
    fn max_operations(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 2, 1, 4, 5] as &[_], 2),
            (&[1, 5, 3, 3, 4, 1, 3, 2, 2, 3], 2),
            (&[5, 3], 1),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_operations(nums.to_vec()), expected);
        }
    }
}
