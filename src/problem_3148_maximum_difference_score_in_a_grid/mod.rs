pub mod dynamic_programming;

pub trait Solution {
    fn max_score(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[9, 5, 7, 3], [8, 9, 6, 1], [6, 7, 14, 3], [2, 5, 3, 1]] as &dyn Matrix<_>,
                9,
            ),
            (&[[4, 3, 2], [3, 2, 1]], -1),
            (&[[4, 9], [5, 2], [3, 1]], 5),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::max_score(grid.to_vec()), expected);
        }
    }
}
