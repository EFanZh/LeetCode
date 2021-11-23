pub mod brute_force;

pub trait Solution {
    fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[4, 3, 8, 4], [9, 5, 1, 9], [2, 7, 6, 2]] as &dyn Matrix<_>, 1),
            (&[[8]], 0),
            (&[[4, 4], [3, 3]], 0),
            (&[[4, 7, 8], [9, 5, 1], [2, 3, 6]], 0),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::num_magic_squares_inside(grid.to_vec()), expected);
        }
    }
}
