pub mod iterative;

pub trait Solution {
    fn maximum_amount(coins: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 1, -1], [1, -2, 3], [2, -3, 4]] as &dyn Matrix<_>, 8),
            (&[[10, 10, 10], [10, 10, 10]], 40),
        ];

        for (coins, expected) in test_cases {
            assert_eq!(S::maximum_amount(coins.to_vec()), expected);
        }
    }
}
