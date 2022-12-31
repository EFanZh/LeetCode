pub mod deque;
pub mod deque_2;

pub trait Solution {
    fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, -1, -3, 5, 3, 6, 7] as &[_], 3), &[3, 3, 5, 5, 6, 7] as &[_]),
            ((&[1], 1), &[1]),
            ((&[-7, -8, 7, 5, 7, 1, 6, 0], 4), &[7, 7, 7, 7, 7]),
            ((&[1, -1], 1), &[1, -1]),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::max_sliding_window(nums.to_vec(), k), expected);
        }
    }
}
