pub mod sliding_window;

pub trait Solution {
    fn max_sum(nums: Vec<i32>, m: i32, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 6, 7, 3, 1, 7] as &[_], 3, 4), 18),
            ((&[5, 9, 9, 2, 4, 5, 4], 1, 3), 23),
            ((&[1, 2, 1, 2, 1, 2, 1], 3, 3), 0),
        ];

        for ((nums, m, k), expected) in test_cases {
            assert_eq!(S::max_sum(nums.to_vec(), m, k), expected);
        }
    }
}
