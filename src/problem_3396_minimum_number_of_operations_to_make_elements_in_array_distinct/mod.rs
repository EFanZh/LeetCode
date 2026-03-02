pub mod iterative;

pub trait Solution {
    fn minimum_operations(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4, 2, 3, 3, 5, 7] as &[_], 2),
            (&[4, 5, 6, 4, 4], 2),
            (&[6, 7, 8, 9], 0),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_operations(nums.to_vec()), expected);
        }
    }
}
