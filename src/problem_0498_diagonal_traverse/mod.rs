pub mod iterative;

pub trait Solution {
    fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[1, 2, 3], [4, 5, 6], [7, 8, 9]] as &dyn Matrix<_>,
                &[1, 2, 4, 7, 5, 3, 6, 8, 9] as &[_],
            ),
            (&[[1, 2], [3, 4]], &[1, 2, 3, 4]),
        ];

        for (mat, expected) in test_cases {
            assert_eq!(S::find_diagonal_order(mat.to_vec()), expected);
        }
    }
}
