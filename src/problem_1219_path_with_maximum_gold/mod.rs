pub mod backtracking;

pub trait Solution {
    fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 6, 0], [5, 8, 7], [0, 9, 0]] as &dyn Matrix<_>, 24),
            (&[[1, 0, 7], [2, 0, 6], [3, 4, 5], [0, 3, 0], [9, 0, 20]], 28),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::get_maximum_gold(grid.to_vec()), expected);
        }
    }
}
