pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_lines(stock_prices: Vec<Vec<i32>>) -> i32 {
        let mut stock_prices = stock_prices
            .into_iter()
            .map(|prices| <[_; 2]>::map(prices.try_into().ok().unwrap(), i32::cast_unsigned))
            .collect::<Vec<_>>();

        stock_prices.sort_unstable_by_key(|&[day, _]| day);

        let mut prev_day = 0;
        let mut prev_price = i32::MAX;
        let mut prev_ratio = (1, 0);
        let mut result = -1;

        for [day, price] in stock_prices {
            let ratio = (price.cast_signed() - prev_price, (day - prev_day).cast_signed());

            result +=
                i32::from(i64::from(prev_ratio.0) * i64::from(ratio.1) != i64::from(prev_ratio.1) * i64::from(ratio.0));

            prev_day = day;
            prev_price = price.cast_signed();
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
