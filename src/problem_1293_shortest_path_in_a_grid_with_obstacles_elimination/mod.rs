pub mod bfs;

pub trait Solution {
    fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[[0, 0, 0], [1, 1, 0], [0, 0, 0], [0, 1, 1], [0, 0, 0]] as &dyn Matrix<_>,
                    1,
                ),
                6,
            ),
            ((&[[0, 1, 1], [1, 1, 1], [1, 0, 0]], 1), -1),
            ((&[[0]], 1), 0),
        ];

        for ((grid, k), expected) in test_cases {
            assert_eq!(S::shortest_path(grid.to_vec(), k), expected);
        }
    }
}
