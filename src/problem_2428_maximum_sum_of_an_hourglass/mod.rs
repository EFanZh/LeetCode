pub mod brute_force;

pub trait Solution {
    fn max_sum(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[6, 2, 1, 3], [4, 2, 1, 5], [9, 2, 8, 7], [4, 1, 2, 9]] as &dyn Matrix<_>,
                30,
            ),
            (&[[1, 2, 3], [4, 5, 6], [7, 8, 9]], 35),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::max_sum(grid.to_vec()), expected);
        }
    }
}
