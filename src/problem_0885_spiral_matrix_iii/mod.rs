pub mod iterative;

pub trait Solution {
    fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((1, 4, 0, 0), &[[0, 0], [0, 1], [0, 2], [0, 3]] as &[_]),
            (
                (5, 6, 1, 4),
                &[
                    [1, 4],
                    [1, 5],
                    [2, 5],
                    [2, 4],
                    [2, 3],
                    [1, 3],
                    [0, 3],
                    [0, 4],
                    [0, 5],
                    [3, 5],
                    [3, 4],
                    [3, 3],
                    [3, 2],
                    [2, 2],
                    [1, 2],
                    [0, 2],
                    [4, 5],
                    [4, 4],
                    [4, 3],
                    [4, 2],
                    [4, 1],
                    [3, 1],
                    [2, 1],
                    [1, 1],
                    [0, 1],
                    [4, 0],
                    [3, 0],
                    [2, 0],
                    [1, 0],
                    [0, 0],
                ],
            ),
            (
                (3, 3, 1, 1),
                &[[1, 1], [1, 2], [2, 2], [2, 1], [2, 0], [1, 0], [0, 0], [0, 1], [0, 2]],
            ),
            (
                (3, 3, 0, 0),
                &[[0, 0], [0, 1], [1, 1], [1, 0], [0, 2], [1, 2], [2, 2], [2, 1], [2, 0]],
            ),
        ];

        for ((rows, cols, r_start, c_start), expected) in test_cases {
            assert_eq!(S::spiral_matrix_iii(rows, cols, r_start, c_start), expected);
        }
    }
}
