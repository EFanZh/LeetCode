pub mod sliding_window;

pub trait Solution {
    fn max_length(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 1, 2, 1, 1, 1] as &[_], 5),
            (&[2, 3, 4, 5, 6], 3),
            (&[1, 2, 3, 1, 4, 5, 1], 5),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_length(nums.to_vec()), expected);
        }
    }
}
