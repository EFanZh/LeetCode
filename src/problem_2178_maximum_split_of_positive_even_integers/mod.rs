pub mod greedy;
pub mod mathematical;

pub trait Solution {
    fn maximum_even_split(final_sum: i64) -> Vec<i64>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;

    pub fn run<S: Solution>() {
        let test_cases = [(12, 3), (7, 0), (28, 4)];

        for (final_sum, expected_length) in test_cases {
            let result = S::maximum_even_split(final_sum);

            assert_eq!(result.len(), expected_length);

            if expected_length != 0 {
                assert_eq!(result.iter().sum::<i64>(), final_sum);
                assert_eq!(HashSet::<_>::from_iter(result).len(), expected_length);
            }
        }
    }
}
