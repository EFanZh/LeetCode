pub mod bfs_and_quick_select;

pub trait Solution {
    fn highest_ranked_k_items(grid: Vec<Vec<i32>>, pricing: Vec<i32>, start: Vec<i32>, k: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[[1, 2, 0, 1], [1, 3, 0, 1], [0, 2, 5, 1]] as &dyn Matrix<_>,
                    [2, 5],
                    [0, 0],
                    3,
                ),
                &[[0, 1], [1, 1], [2, 1]] as &[_],
            ),
            (
                (&[[1, 2, 0, 1], [1, 3, 3, 1], [0, 2, 5, 1]], [2, 3], [2, 3], 2),
                &[[2, 1], [1, 2]],
            ),
            (
                (&[[1, 1, 1], [0, 0, 1], [2, 3, 4]], [2, 3], [0, 0], 3),
                &[[2, 1], [2, 0]],
            ),
            (
                (&[[1, 0, 1], [3, 5, 2], [1, 0, 1]], [2, 5], [1, 1], 9),
                &[[1, 1], [1, 2], [1, 0]],
            ),
        ];

        for ((grid, pricing, start, k), expected) in test_cases {
            assert_eq!(
                S::highest_ranked_k_items(grid.to_vec(), pricing.to_vec(), start.to_vec(), k),
                expected,
            );
        }
    }
}
