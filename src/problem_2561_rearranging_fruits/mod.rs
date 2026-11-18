pub mod greedy;

pub trait Solution {
    fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, 2, 2, 2] as &[_], &[1, 4, 1, 2] as &[_]), 1),
            ((&[2, 3, 4, 1], &[3, 2, 5, 1]), -1),
            (
                (
                    &[84, 80, 43, 8, 80, 88, 43, 14, 100, 88],
                    &[32, 32, 42, 68, 68, 100, 42, 84, 14, 8],
                ),
                48,
            ),
            ((&[80, 43, 80, 88, 43, 100, 88], &[32, 32, 42, 68, 68, 100, 42]), 117),
        ];

        for ((basket1, basket2), expected) in test_cases {
            assert_eq!(S::min_cost(basket1.to_vec(), basket2.to_vec()), expected);
        }
    }
}
