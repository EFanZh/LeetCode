pub mod dynamic_programming;
pub mod use_largest_rectangle_in_histogram;

pub trait Solution {
    fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [(
            &[
                ['1', '0', '1', '0', '0'],
                ['1', '0', '1', '1', '1'],
                ['1', '1', '1', '1', '1'],
                ['1', '0', '0', '1', '0'],
            ] as &dyn Matrix<_>,
            6,
        )];

        for (matrix, expected) in test_cases {
            assert_eq!(S::maximal_rectangle(matrix.to_vec()), expected);
        }
    }
}
