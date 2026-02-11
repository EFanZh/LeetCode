pub mod matrix_multiplication;

pub trait Solution {
    fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    "abcyy",
                    2,
                    [
                        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
                    ],
                ),
                7,
            ),
            (
                (
                    "azbk",
                    1,
                    [
                        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                    ],
                ),
                8,
            ),
        ];

        for ((s, t, nums), expected) in test_cases {
            assert_eq!(
                S::length_after_transformations(s.to_string(), t, nums.to_vec()),
                expected,
            );
        }
    }
}
