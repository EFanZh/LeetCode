pub mod mathematical;

pub trait Solution {
    fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, 3, &[[0, 1], [1, 1]] as &dyn Matrix<_>), 6),
            ((2, 2, &[[1, 1], [0, 0]]), 0),
        ];

        for ((m, n, indices), expected) in test_cases {
            assert_eq!(S::odd_cells(m, n, indices.to_vec()), expected);
        }
    }
}
