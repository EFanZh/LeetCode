pub mod greedy;

pub trait Solution {
    fn buy_choco(prices: Vec<i32>, money: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 2, 2] as &[_], 3), 0), ((&[3, 2, 3], 3), 3)];

        for ((prices, money), expected) in test_cases {
            assert_eq!(S::buy_choco(prices.to_vec(), money), expected);
        }
    }
}
