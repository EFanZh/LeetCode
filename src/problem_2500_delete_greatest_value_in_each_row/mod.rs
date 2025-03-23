pub mod iterative;

pub trait Solution {
    fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [(&[[1, 2, 4], [3, 3, 1]] as &dyn Matrix<_>, 8), (&[[10]], 10)];

        for (grid, expected) in test_cases {
            assert_eq!(S::delete_greatest_value(grid.to_vec()), expected);
        }
    }
}
