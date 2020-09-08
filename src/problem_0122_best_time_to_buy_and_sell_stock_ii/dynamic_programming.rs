pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = i32::min_value();
        let mut sell = 0;

        for price in prices {
            buy = buy.max(sell - price);
            sell = sell.max(buy + price);
        }

        sell
    }
}

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
