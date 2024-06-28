pub mod iterative;

pub trait Solution {
    fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    [3, 4, 5, 1, 3],
                    [3, 3, 4, 2, 3],
                    [20, 30, 200, 40, 10],
                    [1, 5, 5, 4, 1],
                    [4, 3, 2, 2, 5],
                ] as &dyn Matrix<_>,
                &[228, 216, 211] as &[_],
            ),
            (&[[1, 2, 3], [4, 5, 6], [7, 8, 9]], &[20, 9, 8]),
            (&[[7, 7, 7]], &[7]),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::get_biggest_three(grid.to_vec()), expected);
        }
    }
}
