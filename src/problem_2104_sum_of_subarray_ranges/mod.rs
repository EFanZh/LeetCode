pub mod monotonic_stack;

pub trait Solution {
    fn sub_array_ranges(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3] as &[_], 4),
            (&[1, 3, 3], 4),
            (&[4, -2, -3, 4, 1], 59),
            (&[-69, -70, -56, -83, 63], 694),
            (&[0, 29, -52, -45, 41, -86, -9, 96, 73, -77], 6077),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::sub_array_ranges(nums.to_vec()), expected);
        }
    }
}
