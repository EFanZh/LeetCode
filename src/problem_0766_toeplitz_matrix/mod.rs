pub mod iterative;

pub trait Solution {
    fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[&[1, 2, 3, 4] as &[_], &[5, 1, 2, 3], &[9, 5, 1, 2]] as &[&[_]], true),
            (&[&[1, 2], &[2, 2]], false),
        ];

        for (matrix, expected) in test_cases {
            assert_eq!(
                S::is_toeplitz_matrix(matrix.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
