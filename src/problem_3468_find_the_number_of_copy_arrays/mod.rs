pub mod iterative;

pub trait Solution {
    fn count_arrays(original: Vec<i32>, bounds: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4], &[[1, 2], [2, 3], [3, 4], [4, 5]]), 2),
            ((&[1, 2, 3, 4], &[[1, 10], [2, 9], [3, 8], [4, 7]]), 4),
            ((&[1, 2, 1, 2], &[[1, 1], [2, 3], [3, 3], [2, 3]]), 0),
        ];

        for ((original, bounds), expected) in test_cases {
            assert_eq!(S::count_arrays(original.to_vec(), bounds.to_vec()), expected);
        }
    }
}
