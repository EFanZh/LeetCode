pub mod dynamic_programming;

pub trait Solution {
    fn max_points(points: Vec<Vec<i32>>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2, 3], [1, 5, 1], [3, 1, 1]] as &dyn Matrix<_>, 9),
            (&[[1, 5], [2, 3], [4, 2]], 11),
        ];

        for (points, expected) in test_cases {
            assert_eq!(S::max_points(points.to_vec()), expected);
        }
    }
}
