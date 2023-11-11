pub mod sliding_window;
pub mod sliding_window_2;

pub trait Solution {
    fn max_frequency(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 4] as &[_], 5), 3),
            ((&[1, 4, 8, 13], 5), 2),
            ((&[3, 9, 6], 2), 1),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::max_frequency(nums.to_vec(), k), expected);
        }
    }
}
