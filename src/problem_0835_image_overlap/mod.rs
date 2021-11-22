pub mod brute_force;

pub trait Solution {
    fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[[1, 1, 0], [0, 1, 0], [0, 1, 0]] as &dyn Matrix<_>,
                    &[[0, 0, 0], [0, 1, 1], [0, 0, 1]] as &dyn Matrix<_>,
                ),
                3,
            ),
            ((&[[1]], &[[1]]), 1),
            ((&[[0]], &[[0]]), 0),
            ((&[[0, 0], [1, 0]], &[[0, 1], [0, 0]]), 1),
            (
                (
                    &[[0, 1, 1, 1], [0, 1, 1, 1], [0, 1, 1, 1], [0, 0, 0, 0]],
                    &[[0, 0, 0, 0], [1, 1, 1, 0], [1, 1, 1, 0], [1, 1, 1, 0]],
                ),
                9,
            ),
            (
                (&[[1, 1, 0], [1, 1, 0], [1, 1, 0]], &[[0, 1, 1], [0, 1, 1], [0, 1, 1]]),
                6,
            ),
            (
                (&[[1, 1, 1], [1, 1, 1], [0, 0, 0]], &[[0, 0, 0], [1, 1, 1], [1, 1, 1]]),
                6,
            ),
            (
                (&[[1, 1, 0], [1, 1, 0], [0, 0, 0]], &[[0, 0, 0], [0, 1, 1], [0, 1, 1]]),
                4,
            ),
        ];

        for ((img1, img2), expected) in test_cases {
            assert_eq!(S::largest_overlap(img1.to_vec(), img2.to_vec()), expected);
        }
    }
}
