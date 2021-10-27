pub mod iterative;

pub trait Solution {
    fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[3, 0, 8, 4], [2, 4, 5, 7], [9, 2, 6, 3], [0, 3, 1, 0]] as &dyn Matrix<_>,
                35,
            ),
            (&[[0, 0, 0], [0, 0, 0], [0, 0, 0]], 0),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::max_increase_keeping_skyline(grid.to_vec()), expected);
        }
    }
}
