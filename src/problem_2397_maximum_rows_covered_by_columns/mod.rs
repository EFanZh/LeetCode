pub mod brute_force;

pub trait Solution {
    fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 0, 1]] as &dyn Matrix<_>, 2), 3),
            ((&[[1], [0]], 1), 2),
        ];

        for ((matrix, num_select), expected) in test_cases {
            assert_eq!(S::maximum_rows(matrix.to_vec(), num_select), expected,);
        }
    }
}
