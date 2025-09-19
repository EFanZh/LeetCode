pub mod prefix_sums;

pub trait Solution {
    fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5, 6] as &[_], 1), 11),
            ((&[-1, 3, 2, 4, 5], 3), 11),
            ((&[-1, -2, -3, -4], 2), -6),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::maximum_subarray_sum(nums.to_vec(), k), expected);
        }
    }
}
