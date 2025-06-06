pub mod iterative;

pub trait Solution {
    fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 1], [1, 0]] as &dyn Matrix<_>, [0, 1]),
            (&[[0, 0, 0], [0, 1, 1]], [1, 2]),
            (&[[0, 0], [1, 1], [0, 0]], [1, 2]),
        ];

        for (mat, expected) in test_cases {
            assert_eq!(S::row_and_maximum_ones(mat.to_vec()), expected);
        }
    }
}
