pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[5, 2], [1, 6]] as &dyn Matrix<_>, 1), 7),
            ((&[[5, 2], [1, 6]], 2), 5),
            ((&[[5, 2], [1, 6]], 3), 4),
        ];

        for ((matrix, k), expected) in test_cases {
            assert_eq!(S::kth_largest_value(matrix.to_vec(), k), expected);
        }
    }
}
