pub mod sliding_window;

pub trait Solution {
    fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 2, 5, 4] as &[_], 5), 3),
            ((&[1, 2], 2), 1),
            ((&[2, 3, 4, 5], 4), 3),
            ((&[8, 10, 7, 8, 3], 9), 2),
        ];

        for ((nums, threshold), expected) in test_cases {
            assert_eq!(S::longest_alternating_subarray(nums.to_vec(), threshold), expected);
        }
    }
}
