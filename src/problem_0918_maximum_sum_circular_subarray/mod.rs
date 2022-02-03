pub mod iterative;

pub trait Solution {
    fn max_subarray_sum_circular(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, -2, 3, -2] as &[_], 3),
            (&[5, -3, 5], 10),
            (&[-3, -2, -3], -2),
            (&[6, 9, -3], 15),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_subarray_sum_circular(nums.to_vec()), expected);
        }
    }
}
