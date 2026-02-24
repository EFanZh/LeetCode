pub mod modular_arithmetic;

pub trait Solution {
    fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2] as &[_], 1), 3),
            ((&[-1, -2, -3, -4, -5], 4), -10),
            ((&[-5, 1, 2, -3, 4], 2), 4),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::max_subarray_sum(nums.to_vec(), k), expected);
        }
    }
}
