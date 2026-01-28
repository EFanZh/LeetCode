pub mod mathematical;

pub trait Solution {
    fn kth_character(k: i64, operations: Vec<i32>) -> char;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((5, &[0, 0, 0] as &[_]), 'a'),
            ((10, &[0, 1, 0, 1]), 'b'),
            ((3, &[1, 0]), 'a'),
            (
                (
                    67_108_864,
                    &[
                        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    ],
                ),
                'a',
            ),
        ];

        for ((nums, operations), expected) in test_cases {
            assert_eq!(S::kth_character(nums, operations.to_vec()), expected);
        }
    }
}
