pub mod iterative;

pub trait Solution {
    fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 3, 4, 4, 5, 6] as &[_], 4), true),
            ((&[3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11], 3), true),
            ((&[1, 2, 3, 4], 3), false),
            (
                (
                    &[15, 16, 17, 18, 19, 16, 17, 18, 19, 20, 6, 7, 8, 9, 10, 3, 4, 5, 6, 20],
                    5,
                ),
                false,
            ),
            ((&[16, 21, 26, 35], 4), false),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::is_possible_divide(nums.to_vec(), k), expected);
        }
    }
}
