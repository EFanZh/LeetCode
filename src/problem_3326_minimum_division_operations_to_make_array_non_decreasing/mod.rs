pub mod dynamic_programming;
pub mod dynamic_programming_cached;

pub trait Solution {
    fn min_operations(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[25, 7] as &[_], 1),
            (&[7, 7, 6], -1),
            (&[1, 1, 1, 1], 0),
            (&[9, 27, 81, 27, 3], 4),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_operations(nums.to_vec()), expected);
        }
    }
}
