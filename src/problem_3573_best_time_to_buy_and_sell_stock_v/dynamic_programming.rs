pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let k = k.cast_unsigned() as usize;
        let mut cache = vec![(0, i64::MIN / 2, i64::MIN / 2); k].into_boxed_slice();

        assert!(k != 0);

        for price in prices {
            let price = i64::from(price);

            for i in (0..k).rev() {
                let prev = cache
                    .get(i.wrapping_sub(1))
                    .copied()
                    .unwrap_or((0, i64::MIN / 2, i64::MIN / 2));

                let current = &mut cache[i];

                *current = (
                    current.0.max(current.1 + price).max(current.2 - price),
                    current.1.max(prev.0 - price),
                    current.2.max(prev.0 + price),
                );
            }
        }

        cache.iter().fold(i64::MIN, |max, &(x, ..)| max.max(x))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        Self::maximum_profit(prices, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
