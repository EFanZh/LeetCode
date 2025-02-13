pub mod dynamic_programming;

pub trait Solution {
    fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[5, 2, 4], [3, 0, 5], [0, 7, 2]] as &dyn Matrix<_>, 3), 2),
            ((&[[0, 0]], 5), 1),
            ((&[[7, 3, 4, 9], [2, 3, 6, 2], [2, 3, 7, 0]], 1), 10),
        ];

        for ((grid, k), expected) in test_cases {
            assert_eq!(S::number_of_paths(grid.to_vec(), k), expected);
        }
    }
}
