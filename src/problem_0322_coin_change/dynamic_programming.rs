pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut cache = vec![Some(0); amount + 1];

        for i in 1..=amount {
            cache[i] = coins
                .iter()
                .filter_map(|c| i.checked_sub(*c as usize).and_then(|r| cache[r]))
                .min()
                .map(|x| x + 1);
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
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
