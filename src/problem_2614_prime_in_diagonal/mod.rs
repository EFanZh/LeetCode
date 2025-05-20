pub mod iterative;
pub mod sieve_of_eratosthenes;

pub trait Solution {
    fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2, 3], [5, 6, 7], [9, 10, 11]] as &dyn Matrix<_>, 11),
            (&[[1, 2, 3], [5, 17, 7], [9, 11, 10]], 17),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::diagonal_prime(nums.to_vec()), expected);
        }
    }
}
