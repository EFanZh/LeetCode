pub mod dynamic_programming;

pub trait Solution {
    fn count_paths_with_xor_value(grid: Vec<Vec<i32>>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[2, 1, 5], [7, 10, 0], [12, 6, 4]] as &dyn Matrix<_>, 11), 3),
            ((&[[1, 3, 3, 3], [0, 3, 3, 2], [3, 0, 1, 1]], 2), 5),
            ((&[[1, 1, 1, 2], [3, 0, 3, 2], [3, 0, 2, 2]], 10), 0),
        ];

        for ((grid, k), expected) in test_cases {
            assert_eq!(S::count_paths_with_xor_value(grid.to_vec(), k), expected);
        }
    }
}
