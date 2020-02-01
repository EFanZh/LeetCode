pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut cache = vec![Some(0); amount + 1];

        for x in 1..=amount {
            cache[x] = coins
                .iter()
                .filter_map(|c| x.checked_sub(*c as usize).and_then(|r| cache[r]))
                .min()
                .map(|v| v + 1);
        }

        cache.last().unwrap().unwrap_or(-1)
    }
}

impl super::Solution for Solution {
    fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        Self::coin_change(coins, amount)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
