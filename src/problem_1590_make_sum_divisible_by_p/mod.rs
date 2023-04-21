pub mod cumulative_sum;

pub trait Solution {
    fn min_subarray(nums: Vec<i32>, p: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 1, 4, 2] as &[_], 6), 1),
            ((&[6, 3, 5, 2], 9), 2),
            ((&[1, 2, 3], 3), 0),
            ((&[1, 2, 3], 7), -1),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::min_subarray(nums.to_vec(), k), expected);
        }
    }
}
