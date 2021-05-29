pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let mut cache = vec![0; amount + 1];

        cache[0] = 1;

        for coin in coins {
            for i in 1..=amount {
                if let Some(&r) = cache.get(i.wrapping_sub(coin as usize)) {
                    cache[i] += r;
                }
            }
        }

        *cache.last().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn change(amount: i32, coins: Vec<i32>) -> i32 {
        Self::change(amount, coins)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
