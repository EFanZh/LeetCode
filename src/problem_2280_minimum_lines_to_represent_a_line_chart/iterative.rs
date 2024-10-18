pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_lines(stock_prices: Vec<Vec<i32>>) -> i32 {
        let mut stock_prices = stock_prices
            .into_iter()
            .map(|prices| Box::<[_; 2]>::try_from(prices.into_boxed_slice()).unwrap())
            .collect::<Vec<_>>();

        stock_prices.sort_unstable_by_key(|prices| prices[0] as u32);

        let mut prev_day = 0;
        let mut prev_price = i32::MAX;
        let mut prev_ratio = (1, 0);
        let mut result = -1;

        for stock_price in stock_prices {
            let [day, price] = *stock_price;
            let ratio = (price - prev_price, day - prev_day);

            result +=
                i32::from(i64::from(prev_ratio.0) * i64::from(ratio.1) != i64::from(prev_ratio.1) * i64::from(ratio.0));

            prev_day = day;
            prev_price = price;
            prev_ratio = ratio;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_lines(stock_prices: Vec<Vec<i32>>) -> i32 {
        Self::minimum_lines(stock_prices)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
