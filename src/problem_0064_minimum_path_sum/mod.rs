pub mod dynamic_programming;

pub trait Solution {
    fn min_path_sum(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [(&[[1, 3, 1], [1, 5, 1], [4, 2, 1]] as &dyn Matrix<_>, 7)];

        for (grid, expected) in test_cases {
            assert_eq!(S::min_path_sum(grid.to_vec()), expected);
        }
    }
}
