pub mod memoized_dynamic_programming;

pub trait Solution {
    fn split_array_same_average(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4, 5, 6, 7, 8] as &[_], true),
            (&[3, 1], false),
            (&[2, 12, 18, 16, 19, 3], false),
            (&[2, 0, 5, 6, 16, 12, 15, 12, 4], true),
            (&[28, 34, 60, 3, 69, 1], false),
            (&[17, 3, 7, 12, 1], false),
            (&[12, 1, 17, 8, 2], true),
            (&[1, 6, 1], false),
            (&[5, 3, 11, 19, 2], true),
            (
                &[
                    60, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
                    30, 30, 30, 30, 30,
                ],
                false,
            ),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::split_array_same_average(nums.to_vec()), expected);
        }
    }
}
