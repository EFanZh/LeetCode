pub mod dynamic_programming;

pub trait Solution {
    fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, 5, &[[1, 4, 2], [2, 2, 7], [2, 1, 3]] as &dyn Matrix<_>), 19),
            ((4, 6, &[[3, 2, 10], [1, 4, 2], [4, 1, 3]]), 32),
        ];

        for ((m, n, prices), expected) in test_cases {
            assert_eq!(S::selling_wood(m, n, prices.to_vec()), expected);
        }
    }
}
