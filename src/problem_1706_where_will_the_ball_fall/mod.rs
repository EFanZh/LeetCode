pub mod iterative;

pub trait Solution {
    fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    [1, 1, 1, -1, -1],
                    [1, 1, 1, -1, -1],
                    [-1, -1, -1, 1, 1],
                    [1, 1, 1, 1, -1],
                    [-1, -1, -1, -1, -1],
                ] as &dyn Matrix<_>,
                &[1, -1, -1, -1, -1] as &[_],
            ),
            (&[[-1]], &[-1]),
            (
                &[
                    [1, 1, 1, 1, 1, 1],
                    [-1, -1, -1, -1, -1, -1],
                    [1, 1, 1, 1, 1, 1],
                    [-1, -1, -1, -1, -1, -1],
                ],
                &[0, 1, 2, 3, 4, -1],
            ),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::find_ball(grid.to_vec()), expected);
        }
    }
}
