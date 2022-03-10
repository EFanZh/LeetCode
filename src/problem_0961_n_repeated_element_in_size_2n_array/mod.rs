pub mod iterative;

pub trait Solution {
    fn repeated_n_times(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 3] as &[_], 3),
            (&[2, 1, 2, 5, 3, 2], 2),
            (&[5, 1, 5, 2, 5, 3, 5, 4], 5),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::repeated_n_times(nums.to_vec()), expected);
        }
    }
}
