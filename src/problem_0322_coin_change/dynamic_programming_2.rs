pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut cache = vec![0; amount + 1];

        for i in 1..=amount {
            cache[i] = coins
                .iter()
                .filter_map(|c| {
                    i.checked_sub(*c as usize).and_then(|r| {
                        let v = cache[r];

                        if v < 0 {
                            None
                        } else {
                            Some(v)
                        }
                    })
                })
                .min()
                .map_or(-1, |x| x + 1);
        }

        *cache.last().unwrap()
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
