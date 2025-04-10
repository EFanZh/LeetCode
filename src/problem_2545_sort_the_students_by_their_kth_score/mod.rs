pub mod sort_by_key;

pub trait Solution {
    fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[10, 6, 9, 1], [7, 5, 11, 2], [4, 8, 3, 15]] as &dyn Matrix<_>, 2),
                &[[7, 5, 11, 2], [10, 6, 9, 1], [4, 8, 3, 15]] as &dyn Matrix<_>,
            ),
            ((&[[3, 4], [5, 6]], 0), &[[5, 6], [3, 4]]),
        ];

        for ((score, k), expected) in test_cases {
            assert_eq!(S::sort_the_students(score.to_vec(), k), expected);
        }
    }
}
