pub mod iterative;

pub trait Solution {
    fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2], [2, 1], [3, 4], [5, 6]] as &dyn Matrix<_>, 1),
            (&[[1, 2], [1, 2], [1, 1], [1, 2], [2, 2]], 3),
        ];

        for (dominoes, expected) in test_cases {
            assert_eq!(S::num_equiv_domino_pairs(dominoes.to_vec()), expected);
        }
    }
}
