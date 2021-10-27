pub mod memoized_dynamic_programming;
pub mod memoized_dynamic_programming_2;
pub mod memoized_dynamic_programming_3;

pub trait Solution {
    fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[2, 5] as &[_],
                    &[[3, 0, 5], [1, 2, 10]] as &dyn Matrix<_>,
                    &[3, 2] as &[_],
                ),
                14,
            ),
            ((&[2, 3, 4], &[[1, 1, 0, 4], [2, 2, 1, 9]], &[1, 2, 1]), 11),
            ((&[2, 3, 4], &[[1, 1, 0, 4], [2, 2, 1, 9]], &[0, 0, 0]), 0),
            ((&[3, 4], &[[1, 2, 3], [1, 2, 5]], &[2, 2]), 6),
        ];

        for ((price, special, needs), expected) in test_cases {
            assert_eq!(
                S::shopping_offers(price.to_vec(), special.to_vec(), needs.to_vec()),
                expected
            );
        }
    }
}
