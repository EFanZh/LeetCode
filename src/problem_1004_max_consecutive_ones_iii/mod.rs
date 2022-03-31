pub mod sliding_window;

pub trait Solution {
    fn longest_ones(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0] as &[_], 2), 6),
            ((&[0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1], 3), 10),
            ((&[0, 0, 0, 1], 4), 4),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::longest_ones(nums.to_vec(), k), expected);
        }
    }
}
