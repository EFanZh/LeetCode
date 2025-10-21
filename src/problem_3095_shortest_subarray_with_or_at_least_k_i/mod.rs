pub mod sliding_window;

pub trait Solution {
    fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3] as &[_], 2), 1),
            ((&[2, 1, 8], 10), 3),
            ((&[1, 2], 0), 1),
            ((&[1, 2, 32, 21], 55), 3),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::minimum_subarray_length(nums.to_vec(), k), expected);
        }
    }
}
