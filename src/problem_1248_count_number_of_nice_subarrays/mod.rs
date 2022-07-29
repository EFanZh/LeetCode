pub mod sliding_window;
pub mod sliding_window_2;

pub trait Solution {
    fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 1, 2, 1, 1] as &[_], 3), 2),
            ((&[2, 4, 6], 1), 0),
            ((&[2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2), 16),
            (
                (
                    &[
                        91473, 45388, 24720, 35841, 29648, 77363, 86290, 58032, 53752, 87188, 34428, 85343, 19801,
                        73201,
                    ],
                    4,
                ),
                6,
            ),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::number_of_subarrays(nums.to_vec(), k), expected);
        }
    }
}
