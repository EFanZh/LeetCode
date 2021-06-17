pub mod dynamic_programming;

pub trait Solution {
    fn max_profit(prices: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 0, 2] as &[_], 3), (&[1], 0), (&[], 0)];

        for (prices, expected) in test_cases {
            assert_eq!(S::max_profit(prices.to_vec()), expected);
        }
    }
}
