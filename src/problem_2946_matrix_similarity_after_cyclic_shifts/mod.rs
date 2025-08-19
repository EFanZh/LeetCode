pub mod brute_force;

pub trait Solution {
    fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[1, 2, 3], [4, 5, 6], [7, 8, 9]] as &dyn Matrix<_>, 4), false),
            ((&[[1, 2, 1, 2], [5, 5, 5, 5], [6, 3, 6, 3]], 2), true),
            ((&[[2, 2], [2, 2]], 3), true),
        ];

        for ((mat, k), expected) in test_cases {
            assert_eq!(S::are_similar(mat.to_vec(), k), expected);
        }
    }
}
