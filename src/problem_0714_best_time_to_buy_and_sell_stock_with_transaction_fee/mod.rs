pub mod dynamic_programming;

pub trait Solution {
    fn max_profit(prices: Vec<i32>, fee: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 3, 2, 8, 4, 9] as &[_], 2), 8), ((&[1, 3, 7, 5, 10, 3], 3), 6)];

        for ((prices, fee), expected) in test_cases {
            assert_eq!(S::max_profit(prices.to_vec(), fee), expected);
        }
    }
}
