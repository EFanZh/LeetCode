pub mod monotonic_stack;

pub trait Solution {
    fn max_sum_min_product(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 2] as &[_], 14),
            (&[2, 3, 3, 1, 2], 18),
            (&[3, 1, 5, 6, 4, 2], 60),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_sum_min_product(nums.to_vec()), expected);
        }
    }
}
