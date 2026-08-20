pub mod iterative;

pub trait Solution {
    fn minimum_prefix_length(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, -1, 2, 3, 3, 4, 5] as &[_], 4),
            (&[4, 3, -2, -5], 3),
            (&[1, 2, 3, 4], 0),
            (&[-7, 2], 0),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_prefix_length(nums.to_vec()), expected);
        }
    }
}
