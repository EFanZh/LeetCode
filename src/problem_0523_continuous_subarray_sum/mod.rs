pub mod modular_arithmetic;

pub trait Solution {
    fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[23, 2, 4, 6, 7] as &[_], 6), true),
            ((&[23, 2, 6, 4, 7], 6), true),
            ((&[23, 2, 6, 4, 7], 13), false),
            ((&[23, 2, 4, 6, 6], 7), true),
            ((&[0], 1), false),
        ];

        for ((nums, k), expected) in test_cases.iter().copied() {
            assert_eq!(S::check_subarray_sum(nums.to_vec(), k), expected);
        }
    }
}
