pub mod monotonic_stack;

pub trait Solution {
    fn final_prices(prices: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[8, 4, 6, 2, 3] as &[_], &[4, 2, 4, 2, 3] as &[_]),
            (&[1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]),
            (&[10, 1, 1, 6], &[9, 0, 1, 6]),
        ];

        for (prices, expected) in test_cases {
            assert_eq!(S::final_prices(prices.to_vec()), expected);
        }
    }
}
