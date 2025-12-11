pub mod dynamic_programming;

pub trait Solution {
    fn maximum_length(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4] as &[_], 4),
            (&[1, 2, 1, 1, 2, 1, 2], 6),
            (&[1, 3], 2),
            (&[2, 7, 7, 7, 8], 3),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::maximum_length(nums.to_vec()), expected);
        }
    }
}
