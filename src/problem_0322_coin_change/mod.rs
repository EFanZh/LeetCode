pub mod dynamic_programming;

pub trait Solution {
    fn coin_change(coins: Vec<i32>, amount: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = [((&[1, 2, 5] as &[_], 11), 3), ((&[2], 3), -1)];

        for ((coins, amount), expected) in test_cases.iter().copied() {
            assert_eq!(S::coin_change(coins.to_owned(), amount), expected);
        }
    }
}
