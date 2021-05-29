pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy_1 = i32::min_value();
        let mut buy_2 = i32::min_value();
        let mut sell_1 = 0;
        let mut sell_2 = 0;

        for price in prices {
            buy_1 = buy_1.max(-price);
            sell_1 = sell_1.max(buy_1 + price);
            buy_2 = buy_2.max(sell_1 - price);
            sell_2 = sell_2.max(buy_2 + price);
        }

        sell_2
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
