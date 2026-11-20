pub mod sliding_window;

pub trait Solution {
    fn longest_subarray(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[7, 6, 10, 12, 11] as &[_], 3), 3),
            ((&[4, 6, 9, 18], 4), 4),
            ((&[6, 10, 15], 2), 1),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::longest_subarray(nums.to_vec(), k), expected);
        }
    }
}
