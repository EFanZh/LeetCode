pub mod brute_force;

pub trait Solution {
    fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [(
            &[[1, 1, 1], [1, 0, 1], [1, 1, 1]] as &dyn Matrix<_>,
            &[[0, 0, 0], [0, 0, 0], [0, 0, 0]] as &dyn Matrix<_>,
        )];

        for (m, expected) in test_cases {
            assert_eq!(S::image_smoother(m.to_vec()), expected);
        }
    }
}
