pub mod iterator;

pub trait Solution {
    fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2, 3], [3, 2, 1]] as &dyn Matrix<_>, 6),
            (&[[1, 5], [7, 3], [3, 5]], 10),
            (&[[2, 8, 7], [7, 1, 3], [1, 9, 5]], 17),
        ];

        for (accounts, expected) in test_cases {
            assert_eq!(S::maximum_wealth(accounts.to_vec()), expected);
        }
    }
}
