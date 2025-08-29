pub mod mathematical;

pub trait Solution {
    fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 3], [2, 2]] as &dyn Matrix<_>, [2, 4]),
            (&[[9, 1, 7], [8, 9, 2], [3, 4, 6]], [9, 5]),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::find_missing_and_repeated_values(grid.to_vec()), expected);
        }
    }
}
