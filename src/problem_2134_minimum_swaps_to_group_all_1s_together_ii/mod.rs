pub mod sliding_window;

pub trait Solution {
    fn min_swaps(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[0, 1, 0, 1, 1, 0, 0] as &[_], 1),
            (&[0, 1, 1, 1, 0, 0, 1, 1, 0], 2),
            (&[1, 1, 0, 0, 1], 0),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_swaps(nums.to_vec()), expected);
        }
    }
}
