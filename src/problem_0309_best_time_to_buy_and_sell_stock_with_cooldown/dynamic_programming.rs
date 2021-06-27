pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = i32::MIN;
        let mut sell = 0;
        let mut cooldown = 0;

        for price in prices {
            let new_buy = buy.max(cooldown - price);
            let new_sell = buy + price;
            let new_cooldown = cooldown.max(sell);

            buy = new_buy;
            sell = new_sell;
            cooldown = new_cooldown;
        }

        sell.max(cooldown)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        Self::max_profit(prices)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
