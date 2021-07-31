pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut sold = 0;
        let mut bought = i32::MIN;

        for price in prices {
            let new_sold = sold.max(bought + price);
            let new_bought = bought.max(sold - (price + fee));

            sold = new_sold;
            bought = new_bought;
        }

        sold
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        Self::max_profit(prices, fee)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
