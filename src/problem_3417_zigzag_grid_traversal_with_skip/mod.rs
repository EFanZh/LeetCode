pub mod iterative;

pub trait Solution {
    fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2], [3, 4]] as &dyn Matrix<_>, &[1, 4] as &[_]),
            (&[[2, 1], [2, 1], [2, 1]], &[2, 1, 2]),
            (&[[1, 2, 3], [4, 5, 6], [7, 8, 9]], &[1, 3, 5, 7, 9]),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::zigzag_traversal(grid.to_vec()), expected);
        }
    }
}
