pub mod iterative;

pub trait Solution {
    fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4] as &[_], 2, 2), &[[1, 2], [3, 4]] as &dyn Matrix<_>),
            ((&[1, 2, 3], 1, 3), &[[1, 2, 3]]),
            ((&[1, 2], 1, 1), &[] as &[[i32; 0]; 0] as &dyn Matrix<_>),
        ];

        for ((original, m, n), expected) in test_cases {
            assert_eq!(S::construct2_d_array(original.to_vec(), m, n), expected);
        }
    }
}
