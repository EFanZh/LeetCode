pub mod monotonic_stack;

pub trait Solution {
    fn max_width_ramp(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[6, 0, 8, 2, 1, 5] as &[_], 4), (&[9, 8, 1, 0, 1, 9, 4, 0, 4, 1], 7)];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_width_ramp(nums.to_vec()), expected);
        }
    }
}
