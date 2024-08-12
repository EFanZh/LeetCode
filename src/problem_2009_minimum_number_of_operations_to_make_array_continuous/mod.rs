pub mod iterative;

pub trait Solution {
    fn min_operations(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[4, 2, 5, 3] as &[_], 0),
            (&[1, 2, 3, 5, 6], 1),
            (&[1, 10, 100, 1000], 3),
            (&[8, 5, 9, 9, 8, 4], 2),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_operations(nums.to_vec()), expected);
        }
    }
}
