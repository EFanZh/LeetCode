pub mod monotonic_deque;

pub trait Solution {
    fn max_result(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, -1, -2, 4, -7, 3] as &[_], 2), 7),
            ((&[10, -5, -2, 4, 0, 3], 3), 17),
            ((&[1, -5, -20, 4, -1, 3, -6, -3], 2), 0),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::max_result(nums.to_vec(), k), expected);
        }
    }
}
