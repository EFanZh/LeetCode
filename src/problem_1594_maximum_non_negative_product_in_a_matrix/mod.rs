pub mod dynamic_programming;

pub trait Solution {
    fn max_product_path(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[-1, -2, -3], [-2, -3, -3], [-3, -3, -2]] as &dyn Matrix<_>, -1),
            (&[[1, -2, 1], [1, -2, 1], [3, -4, 1]], 8),
            (&[[1, 3], [0, -4]], 0),
            (&[[1, 4, 4, 0], [-2, 0, 0, 1], [1, -1, 1, 1]], 2),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::max_product_path(grid.to_vec()), expected);
        }
    }
}
