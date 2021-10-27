pub mod brute_force;

pub trait Solution {
    fn island_perimeter(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[0, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [1, 1, 0, 0]] as &dyn Matrix<_>,
                16,
            ),
            (&[[1]], 4),
            (&[[1, 0]], 4),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::island_perimeter(grid.to_vec()), expected);
        }
    }
}
