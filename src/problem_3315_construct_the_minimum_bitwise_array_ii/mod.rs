pub mod mathematical;

pub trait Solution {
    fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 3, 5, 7] as &[_], &[-1, 1, 4, 3] as &[_]),
            (&[11, 13, 31], &[9, 12, 15]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_bitwise_array(nums.to_vec()), expected);
        }
    }
}
