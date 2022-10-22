pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn count_negatives(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[4, 3, 2, -1], [3, 2, 1, -1], [1, 1, -1, -2], [-1, -1, -2, -3]] as &dyn Matrix<_>,
                8,
            ),
            (&[[3, 2], [1, 0]], 0),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::count_negatives(grid.to_vec()), expected);
        }
    }
}
