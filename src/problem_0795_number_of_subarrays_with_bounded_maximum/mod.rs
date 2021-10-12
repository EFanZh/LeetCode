pub mod iterative;

pub trait Solution {
    fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 1, 4, 3] as &[_], 2, 3), 3),
            ((&[2, 9, 2, 5, 6], 2, 8), 7),
            ((&[73, 55, 36, 5, 55, 14, 9, 7, 72, 52], 32, 69), 22),
        ];

        for ((nums, left, right), expected) in test_cases {
            assert_eq!(S::num_subarray_bounded_max(nums.to_vec(), left, right), expected);
        }
    }
}
