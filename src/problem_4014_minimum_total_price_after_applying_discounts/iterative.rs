pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_price(prices: Vec<i32>, discounts: Vec<i32>) -> f64 {
        let prices = &mut *prices.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let prices_len = prices.len();
        let discounts = &mut *discounts.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let discounts_len = discounts.len();
        let rev_cmp = |lhs: &_, rhs: &_| u32::cmp(rhs, lhs);

        let ((left, right), discounts) = if discounts_len < prices_len {
            prices.select_nth_unstable_by(discounts_len, rev_cmp);

            (prices.split_at_mut(discounts_len), discounts)
        } else {
            (
                (prices, Default::default()),
                if prices_len < discounts_len {
                    discounts.select_nth_unstable_by(prices_len, rev_cmp).0
                } else {
                    discounts
                },
            )
        };

        left.sort_unstable();
        discounts.sort_unstable();

        let mut result_100x = right.iter().copied().map(u64::from).sum::<u64>() * 100;

        left.iter()
            .zip(&*discounts)
            .for_each(|(&price, &discount)| result_100x += u64::from(price * (100 - discount)));

        #[expect(clippy::cast_precision_loss, reason = "by design")]
        let result = (result_100x as f64) / 100.0;

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_price(prices: Vec<i32>, discounts: Vec<i32>) -> f64 {
        Self::min_price(prices, discounts)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
