pub mod bfs;

pub trait Solution {
    fn min_flips(mat: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 0], [0, 1]] as &dyn Matrix<_>, 3),
            (&[[0]], 0),
            (&[[1, 0, 0], [1, 0, 0]], -1),
        ];

        for (mat, expected) in test_cases {
            assert_eq!(S::min_flips(mat.to_vec()), expected);
        }
    }
}
