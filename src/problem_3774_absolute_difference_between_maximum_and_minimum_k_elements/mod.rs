pub mod quick_select;

pub trait Solution {
    fn abs_difference(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[5, 2, 2, 4] as &[_], 2), 5),
            ((&[100], 1), 0),
            ((&[1, 1, 2, 2], 3), 1),
            ((&[1, 1], 2), 0),
            (
                (
                    &[
                        43, 32, 94, 42, 91, 9, 25, 73, 29, 31, 19, 70, 58, 12, 11, 41, 66, 63, 14, 39,
                    ],
                    18,
                ),
                165,
            ),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::abs_difference(nums.to_vec(), k), expected);
        }
    }
}
