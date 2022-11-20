pub mod mathematical;

pub trait Solution {
    fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[3, 7, 8], [9, 11, 13], [15, 16, 17]] as &dyn Matrix<_>, &[15] as &[_]),
            (&[[1, 10, 4, 2], [9, 3, 8, 7], [15, 16, 17, 12]], &[12]),
            (&[[7, 8], [1, 2]], &[7]),
            (&[[3, 6], [7, 1], [5, 2], [4, 8]], &[]),
        ];

        for (matrix, expected) in test_cases {
            assert_eq!(S::lucky_numbers(matrix.to_vec()), expected);
        }
    }
}
