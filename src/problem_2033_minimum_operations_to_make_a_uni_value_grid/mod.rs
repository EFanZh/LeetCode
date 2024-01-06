pub mod median;

pub trait Solution {
    fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[2, 4], [6, 8]] as &dyn Matrix<_>, 2), 4),
            ((&[[1, 5], [2, 3]], 1), 5),
            ((&[[1, 2], [3, 4]], 2), -1),
        ];

        for ((grid, x), expected) in test_cases {
            assert_eq!(S::min_operations(grid.to_vec(), x), expected);
        }
    }
}
