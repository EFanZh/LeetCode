pub mod quick_select;

pub trait Solution {
    fn max_sum(grid: Vec<Vec<i32>>, limits: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[1, 2], [3, 4]] as &dyn Matrix<_>, &[1, 2] as &[_], 2), 7),
            ((&[[5, 3, 7], [8, 2, 6]], &[2, 2], 3), 21),
        ];

        for ((grid, limits, k), expected) in test_cases {
            assert_eq!(S::max_sum(grid.to_vec(), limits.to_vec(), k), expected);
        }
    }
}
