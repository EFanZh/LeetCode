pub mod level_by_level;
pub mod rotate_matrix;

pub trait Solution {
    fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[&[1, 2, 3] as &[_], &[4, 5, 6], &[7, 8, 9]] as &[&[_]],
                &[1, 2, 3, 6, 9, 8, 7, 4, 5] as &[_],
            ),
            (
                &[&[1, 2, 3, 4], &[5, 6, 7, 8], &[9, 10, 11, 12]],
                &[1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
            ),
            (
                &[&[2, 3, 4], &[5, 6, 7], &[8, 9, 10], &[11, 12, 13]],
                &[2, 3, 4, 7, 10, 13, 12, 11, 8, 5, 6, 9],
            ),
            (
                &[
                    &[1, 11],
                    &[2, 12],
                    &[3, 13],
                    &[4, 14],
                    &[5, 15],
                    &[6, 16],
                    &[7, 17],
                    &[8, 18],
                    &[9, 19],
                    &[10, 20],
                ],
                &[1, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 10, 9, 8, 7, 6, 5, 4, 3, 2],
            ),
        ];

        for (matrix, expected) in test_cases {
            assert_eq!(
                S::spiral_order(matrix.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
