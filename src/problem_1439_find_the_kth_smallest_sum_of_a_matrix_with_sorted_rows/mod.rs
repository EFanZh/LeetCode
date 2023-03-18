pub mod binary_heap;

pub trait Solution {
    fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[1, 3, 11], [2, 4, 6]] as &dyn Matrix<_>, 5), 7),
            ((&[[1, 3, 11], [2, 4, 6]], 9), 17),
            ((&[[1, 10, 10], [1, 4, 5], [2, 3, 6]], 7), 9),
        ];

        for ((mat, k), expected) in test_cases {
            assert_eq!(S::kth_smallest(mat.to_vec(), k), expected);
        }
    }
}
