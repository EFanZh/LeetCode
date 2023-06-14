pub mod count_bits;

pub trait Solution {
    fn min_operations(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 5] as &[_], 5), (&[2, 2], 3), (&[4, 2, 5], 6)];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_operations(nums.to_vec()), expected);
        }
    }
}
