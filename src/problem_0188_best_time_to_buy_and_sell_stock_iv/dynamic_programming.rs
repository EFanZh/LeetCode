pub struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;

        if k == 0 {
            0
        } else if k < prices.len() / 2 {
            let mut cache = vec![(i32::min_value(), 0); k];

            for price in prices {
                let (first_buy, first_sell) = &mut cache[0];

                *first_buy = (*first_buy).max(-price);
                *first_sell = (*first_sell).max(*first_buy + price);

                for i in 1..k {
                    let prev_sell = cache[i - 1].1;
                    let (buy, sell) = &mut cache[i];

                    *buy = (*buy).max(prev_sell - price);
                    *sell = (*sell).max(*buy + price);
                }
            }

            cache[k - 1].1
        } else {
            let mut buy = i32::min_value();
            let mut sell = 0;

            for price in prices {
                buy = buy.max(sell - price);
                sell = sell.max(buy + price);
            }

            sell
        }
    }
}

impl super::Solution for Solution {
    fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        Self::max_profit(k, prices)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
