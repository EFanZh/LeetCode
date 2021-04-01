pub mod eliminate_row_or_column;
pub mod eliminate_row_or_column_2;

pub trait Solution {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[
                        &[1, 4, 7, 11, 15] as &[_],
                        &[2, 5, 8, 12, 19],
                        &[3, 6, 9, 16, 22],
                        &[10, 13, 14, 17, 24],
                        &[18, 21, 23, 26, 30],
                    ] as &[&[_]],
                    5,
                ),
                true,
            ),
            (
                (
                    &[
                        &[1, 4, 7, 11, 15],
                        &[2, 5, 8, 12, 19],
                        &[3, 6, 9, 16, 22],
                        &[10, 13, 14, 17, 24],
                        &[18, 21, 23, 26, 30],
                    ],
                    20,
                ),
                false,
            ),
        ];

        for ((matrix, target), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::search_matrix(matrix.iter().copied().map(<[_]>::to_vec).collect(), target),
                expected
            );
        }
    }
}
