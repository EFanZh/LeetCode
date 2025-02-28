pub mod sliding_window;

pub trait Solution {
    fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 5, 4, 2, 9, 9, 9] as &[_], 3), 15), ((&[4, 4, 4], 3), 0)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::maximum_subarray_sum(nums.to_vec(), k), expected);
        }
    }
}
