pub mod dynamic_programming;

pub trait Solution {
    fn maximum_profit(prices: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 7, 9, 8, 2] as &[_], 2), 14),
            ((&[12, 16, 19, 19, 8, 1, 19, 13, 9], 3), 36),
        ];

        for ((prices, k), expected) in test_cases {
            assert_eq!(S::maximum_profit(prices.to_vec(), k), expected);
        }
    }
}
