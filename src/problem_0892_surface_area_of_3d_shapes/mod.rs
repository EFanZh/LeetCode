pub mod iterative;

pub trait Solution {
    fn surface_area(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[2]] as &dyn Matrix<_>, 10),
            (&[[1, 2], [3, 4]], 34),
            (&[[1, 0], [0, 2]], 16),
            (&[[1, 1, 1], [1, 0, 1], [1, 1, 1]], 32),
            (&[[2, 2, 2], [2, 1, 2], [2, 2, 2]], 46),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::surface_area(grid.to_vec()), expected);
        }
    }
}
