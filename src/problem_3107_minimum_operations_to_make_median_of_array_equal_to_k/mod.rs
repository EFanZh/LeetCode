pub mod greedy;

pub trait Solution {
    fn min_operations_to_make_median_k(nums: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 5, 6, 8, 5] as &[_], 4), 2),
            ((&[2, 5, 6, 8, 5], 7), 3),
            ((&[1, 2, 3, 4, 5, 6], 4), 0),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::min_operations_to_make_median_k(nums.to_vec(), k), expected);
        }
    }
}
