pub mod iterative;

pub trait Solution {
    fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4, 5] as &[_], &[-3, -1, 1, 3, 5] as &[_]),
            (&[3, 2, 3, 4, 2], &[-2, -1, 0, 2, 3]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::distinct_difference_array(nums.to_vec()), expected);
        }
    }
}
