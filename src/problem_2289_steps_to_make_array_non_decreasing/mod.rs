pub mod monotonic_stack;

pub trait Solution {
    fn total_steps(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11] as &[_], 3),
            (&[4, 5, 7, 7, 13], 0),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::total_steps(nums.to_vec()), expected);
        }
    }
}
