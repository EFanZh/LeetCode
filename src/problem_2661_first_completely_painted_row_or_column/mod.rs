pub mod greedy;

pub trait Solution {
    fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 4, 2] as &[_], &[[1, 4], [2, 3]] as &dyn Matrix<_>), 2),
            ((&[2, 8, 7, 4, 1, 3, 5, 6, 9], &[[3, 2, 5], [1, 4, 6], [8, 7, 9]]), 3),
        ];

        for ((arr, mat), expected) in test_cases {
            assert_eq!(S::first_complete_index(arr.to_vec(), mat.to_vec()), expected);
        }
    }
}
