pub mod dynamic_programming;

pub trait Solution {
    fn max_size_slices(slices: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4, 5, 6] as &[_], 10),
            (&[8, 9, 8, 6, 1, 1], 16),
            (&[9, 8, 1, 7, 7, 9, 5, 10, 7, 9, 3, 8, 3, 4, 8], 45),
            (&[3, 7, 8, 6, 3, 10, 4, 2, 9], 27),
            (
                &[
                    10, 1, 1, 2, 1, 10, 3, 10, 2, 8, 4, 10, 8, 8, 2, 9, 9, 9, 10, 10, 7, 6, 5, 6, 3, 8, 2, 6, 8, 10,
                ],
                92,
            ),
        ];

        for (slices, expected) in test_cases {
            assert_eq!(S::max_size_slices(slices.to_vec()), expected);
        }
    }
}
