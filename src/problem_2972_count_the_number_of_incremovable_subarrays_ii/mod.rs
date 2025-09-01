pub mod iterative;

pub trait Solution {
    fn incremovable_subarray_count(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 4] as &[_], 10), (&[6, 5, 7, 8], 7), (&[8, 7, 6, 6], 3)];

        for (nums, expected) in test_cases {
            assert_eq!(S::incremovable_subarray_count(nums.to_vec()), expected);
        }
    }
}
