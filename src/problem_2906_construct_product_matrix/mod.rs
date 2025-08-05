pub mod iterative;

pub trait Solution {
    fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[1, 2], [3, 4]] as &dyn Matrix<_>,
                &[[24, 12], [8, 6]] as &dyn Matrix<_>,
            ),
            (&[[12345], [2], [1]], &[[2], [0], [0]]),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::construct_product_matrix(grid.to_vec()), expected);
        }
    }
}
