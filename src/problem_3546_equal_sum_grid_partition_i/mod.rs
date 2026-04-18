pub mod iterative;

pub trait Solution {
    fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [(&[[1, 4], [2, 3]] as &dyn Matrix<_>, true), (&[[1, 3], [2, 4]], false)];

        for (grid, expected) in test_cases {
            assert_eq!(S::can_partition_grid(grid.to_vec()), expected);
        }
    }
}
