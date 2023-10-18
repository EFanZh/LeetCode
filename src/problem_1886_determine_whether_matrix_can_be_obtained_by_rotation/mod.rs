pub mod brute_force;

pub trait Solution {
    fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[0, 1], [1, 0]] as &dyn Matrix<_>, &[[1, 0], [0, 1]] as &dyn Matrix<_>),
                true,
            ),
            ((&[[0, 1], [1, 1]], &[[1, 0], [0, 1]]), false),
            (
                (&[[0, 0, 0], [0, 1, 0], [1, 1, 1]], &[[1, 1, 1], [0, 1, 0], [0, 0, 0]]),
                true,
            ),
        ];

        for ((mat, target), expected) in test_cases {
            assert_eq!(S::find_rotation(mat.to_vec(), target.to_vec()), expected);
        }
    }
}
