pub mod hash_map;

pub trait Solution {
    fn equal_pairs(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[3, 2, 1], [1, 7, 6], [2, 7, 7]] as &dyn Matrix<_>, 1),
            (&[[3, 1, 2, 2], [1, 4, 4, 5], [2, 4, 2, 2], [2, 4, 2, 2]], 3),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::equal_pairs(grid.to_vec()), expected);
        }
    }
}
