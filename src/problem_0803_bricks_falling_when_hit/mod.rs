pub mod reverse_time_and_union_find;

pub trait Solution {
    fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[[1, 0, 0, 0], [1, 1, 1, 0]] as &dyn Matrix<_>,
                    &[[1, 0]] as &dyn Matrix<_>,
                ),
                &[2] as &[_],
            ),
            ((&[[1, 0, 0, 0], [1, 1, 0, 0]], &[[1, 1], [1, 0]]), &[0, 0]),
            (
                (&[[1], [1], [1], [1], [1]], &[[3, 0], [4, 0], [1, 0], [2, 0], [0, 0]]),
                &[1, 0, 1, 0, 0],
            ),
            (
                (&[[1, 1, 1], [0, 1, 0], [0, 0, 0]], &[[0, 2], [2, 0], [0, 1], [1, 2]]),
                &[0, 0, 1, 0],
            ),
            (
                (
                    &[
                        [1, 1, 0, 1, 0],
                        [1, 1, 0, 1, 1],
                        [0, 0, 0, 1, 1],
                        [0, 0, 0, 1, 0],
                        [0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    &[[5, 1], [1, 3]],
                ),
                &[0, 4],
            ),
        ];

        for ((grid, hits), expected) in test_cases {
            assert_eq!(S::hit_bricks(grid.to_vec(), hits.to_vec()), expected);
        }
    }
}
