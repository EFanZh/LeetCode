pub mod iterative;

pub trait Solution {
    fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 0, 2], [1, 0, 2]] as &dyn Matrix<_>, true),
            (&[[1, 1, 1], [0, 0, 0]], false),
            (&[[1], [2], [3]], false),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::satisfies_conditions(grid.to_vec()), expected);
        }
    }
}
