pub mod dynamic_programming;

pub trait Solution {
    fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[1, 0, 0], [0, 1, 1], [0, 1, 1]] as &dyn Matrix<_>,
                &[[0, 0, 0, 0], [1, 1, 2, 2]] as &[_],
            ),
            (&[[1, 1], [1, 1]], &[[0, 0, 1, 1]]),
            (&[[0]], &[]),
            (&[[0, 1], [1, 0]], &[[0, 1, 0, 1], [1, 0, 1, 0]]),
            (&[[1, 0], [1, 0]], &[[0, 0, 1, 0]]),
            (&[[1, 1], [0, 0]], &[[0, 0, 0, 1]]),
        ];

        for (land, expected) in test_cases {
            assert_eq!(S::find_farmland(land.to_vec()), expected);
        }
    }
}
