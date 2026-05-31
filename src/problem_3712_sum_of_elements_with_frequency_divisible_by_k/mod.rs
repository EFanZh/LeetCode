pub mod iterative;

pub trait Solution {
    fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 2, 3, 3, 3, 3, 4] as &[_], 2), 16),
            ((&[1, 2, 3, 4, 5], 2), 0),
            ((&[4, 4, 4, 1, 2, 3], 3), 12),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::sum_divisible_by_k(nums.to_vec(), k), expected);
        }
    }
}
