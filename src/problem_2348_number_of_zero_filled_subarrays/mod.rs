pub mod iterative;

pub trait Solution {
    fn zero_filled_subarray(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 3, 0, 0, 2, 0, 0, 4] as &[_], 6),
            (&[0, 0, 0, 2, 0, 0], 9),
            (&[2, 10, 2019], 0),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::zero_filled_subarray(nums.to_vec()), expected);
        }
    }
}
