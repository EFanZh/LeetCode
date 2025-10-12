pub mod bfs;

pub trait Solution {
    fn find_max_fish(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[0, 2, 1, 0], [4, 0, 0, 3], [1, 0, 0, 4], [0, 3, 2, 0]] as &dyn Matrix<_>,
                7,
            ),
            (&[[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]], 1),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::find_max_fish(grid.to_vec()), expected);
        }
    }
}
