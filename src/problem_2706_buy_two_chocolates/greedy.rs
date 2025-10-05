pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let prices = prices.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let money = money as u32;
        let mut min_1 = u32::MAX / 2;
        let mut min_2 = u32::MAX / 2;

        for price in prices {
            if price < min_2 {
                if price < min_1 {
                    min_2 = min_1;
                    min_1 = price;
                } else {
                    min_2 = price;
                }
            }
        }

        money.checked_sub(min_1 + min_2).unwrap_or(money) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        Self::buy_choco(prices, money)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
