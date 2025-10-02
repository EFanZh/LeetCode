pub mod prefix_sums;

pub trait Solution {
    fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[7, 6, 3], [6, 6, 1]] as &dyn Matrix<_>, 18), 4),
            ((&[[7, 2, 9], [1, 5, 0], [2, 6, 6]], 20), 6),
            ((&[[1, 7], [5, 10], [4, 1], [2, 4]], 9), 3),
        ];

        for ((grid, k), expected) in test_cases {
            assert_eq!(S::count_submatrices(grid.to_vec(), k), expected);
        }
    }
}
