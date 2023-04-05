pub mod dynamic_programming;

pub trait Solution {
    fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[0, 0, 0, 0, 0] as &[_],
                    &[[1, 10], [10, 1], [10, 1], [1, 10], [5, 1]] as &dyn Matrix<_>,
                    5,
                    2,
                    3,
                ),
                9,
            ),
            (
                (&[0, 2, 1, 2, 0], &[[1, 10], [10, 1], [10, 1], [1, 10], [5, 1]], 5, 2, 3),
                11,
            ),
            (
                (&[3, 1, 2, 3], &[[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]], 4, 3, 3),
                -1,
            ),
            ((&[0, 0, 0, 1], &[[1, 5], [4, 1], [1, 3], [4, 4]], 4, 2, 4), 12),
        ];

        for ((houses, cost, m, n, target), expected) in test_cases {
            assert_eq!(S::min_cost(houses.to_vec(), cost.to_vec(), m, n, target), expected);
        }
    }
}
