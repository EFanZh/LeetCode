pub mod dynamic_programming;

pub trait Solution {
    fn can_distribute(nums: Vec<i32>, quantity: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4] as &[_], &[2] as &[_]), false),
            ((&[1, 2, 3, 3], &[2]), true),
            ((&[1, 1, 2, 2], &[2, 2]), true),
            (
                (
                    &[
                        1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14,
                        15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24, 25, 25, 26, 26,
                        27, 27, 28, 28, 29, 29, 30, 30, 31, 31, 32, 32, 33, 33, 34, 34, 35, 35, 36, 36, 37, 37, 38, 38,
                        39, 39, 40, 40, 41, 41, 42, 42, 43, 43, 44, 44, 45, 45, 46, 46, 47, 47, 48, 48, 49, 49, 50, 50,
                    ],
                    &[2, 2, 2, 2, 2, 2, 2, 2, 2, 3],
                ),
                false,
            ),
            ((&[1, 1, 2, 2, 1], &[2, 2, 1]), true),
            ((&[1, 1, 1, 1, 2, 2, 2], &[3, 2, 2]), true),
        ];

        for ((nums, quantity), expected) in test_cases {
            assert_eq!(S::can_distribute(nums.to_vec(), quantity.to_vec()), expected);
        }
    }
}
