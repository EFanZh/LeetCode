pub mod dynamic_programming;

pub trait Solution {
    fn count_special_subsequences(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[0, 1, 2, 2] as &[_], 3),
            (&[2, 2, 0, 0], 0),
            (&[0, 1, 2, 0, 1, 2], 7),
            (
                &[
                    2, 0, 0, 1, 2, 0, 2, 1, 2, 2, 2, 2, 2, 2, 1, 0, 2, 0, 2, 2, 1, 1, 2, 1, 1, 0, 1, 0, 2, 1, 2, 0, 1,
                    2, 1, 2, 0, 0, 2, 0, 2, 1, 0, 0, 0, 1, 1, 0, 1, 2, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 1, 2, 0, 1, 2,
                    1, 0, 1, 0, 1, 1, 2, 0, 0, 0, 1, 1, 1, 2, 1, 0, 0, 0, 0, 2, 2, 1, 1, 1, 2, 1, 2, 0, 1, 0, 0, 1, 0,
                    1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 2, 1, 1, 0, 0, 2, 0, 2, 2, 1, 0, 2, 2, 2, 0, 0, 1, 2, 1, 1, 2, 1, 2,
                    1, 1, 1, 1, 2, 0, 2, 0, 1, 0, 2, 0, 2, 0, 0, 1, 2, 1, 1, 0, 2, 0, 1, 0, 2, 1, 1, 0, 0, 1, 1, 0, 1,
                    0, 2, 1, 2, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 0, 2, 2, 2, 1, 0, 0, 0, 1, 1, 0, 2, 1, 1, 2, 1, 0, 1,
                    0, 0, 2, 1, 0, 2, 1, 1, 1, 0, 2, 2, 2, 1, 1, 0, 2, 1, 0, 1, 0, 0, 2, 1, 2, 0, 0, 2, 1, 1, 0, 1, 1,
                    1, 1, 1, 1, 2, 1, 2, 1, 1, 0, 0, 2, 0, 1, 0, 0, 0, 2, 1, 1, 2, 1, 1, 0, 0, 2, 0, 1, 2, 1, 1, 1, 0,
                    2, 1, 1, 0, 0, 1, 2, 1, 1, 1, 2, 0, 1, 0, 2, 2, 2, 2, 0, 2, 2, 2, 2, 0, 1, 0, 2, 1, 1, 2, 1, 2, 1,
                    2, 2, 1, 2, 0, 2, 2, 0, 0, 2, 0, 1, 2, 2, 2, 2, 0, 1, 1, 0, 0, 1, 2, 0, 2, 2, 1, 1, 1, 0, 1, 1, 0,
                    1, 1, 2, 0, 2, 0, 1, 1, 0, 0, 2, 0, 2, 1, 2, 1, 1, 0, 0, 2, 2, 1, 0, 0, 0, 0, 0, 1, 2, 0, 0, 0, 1,
                    1, 1, 1, 2, 0, 1, 1, 0, 0, 0, 1, 0, 2, 1, 0, 2, 0, 1, 0, 2, 0, 1, 1, 2, 2, 0, 1, 0, 0, 2, 1, 1, 2,
                    1, 1, 2, 2, 0, 1, 2, 0, 2, 2, 0, 0, 2, 0, 0, 2, 2, 2, 2, 2, 0, 1, 2, 0, 1, 0, 1, 1, 0, 1, 1, 2, 1,
                    0, 1, 2, 1, 2, 1, 1, 1, 2, 2, 1, 0, 1, 1, 2, 2, 2, 0, 0, 1, 2, 1, 2, 2, 1, 1, 0, 2, 2, 2, 2, 2, 1,
                    1, 2, 2, 2, 0, 0, 0, 2, 2, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 2, 0,
                ],
                782_041_255,
            ),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_special_subsequences(nums.to_vec()), expected);
        }
    }
}
