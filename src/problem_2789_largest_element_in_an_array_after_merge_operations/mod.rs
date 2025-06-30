pub mod monotonic_stack;

pub trait Solution {
    fn max_array_value(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3, 7, 9, 3] as &[_], 21), (&[5, 3, 3], 11)];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_array_value(nums.to_vec()), expected);
        }
    }
}
