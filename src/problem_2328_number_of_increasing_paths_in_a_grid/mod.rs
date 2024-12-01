pub mod bfs;

pub trait Solution {
    fn count_paths(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [(&[[1, 1], [3, 4]] as &dyn Matrix<_>, 8), (&[[1], [2]], 3)];

        for (grid, expected) in test_cases {
            assert_eq!(S::count_paths(grid.to_vec()), expected);
        }
    }
}
