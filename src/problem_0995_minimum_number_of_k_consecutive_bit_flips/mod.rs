pub mod greedy_sliding_window;
pub mod greedy_sliding_window_inplace;

pub trait Solution {
    fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[0, 1, 0] as &[_], 1), 2),
            ((&[1, 1, 0], 2), -1),
            ((&[0, 0, 0, 1, 0, 1, 1, 0], 3), 3),
            ((&[0, 1], 2), -1),
            ((&[0, 1, 1], 2), -1),
            ((&[1, 1, 1, 1, 1, 0], 4), -1),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::min_k_bit_flips(nums.to_vec(), k), expected);
        }
    }
}
