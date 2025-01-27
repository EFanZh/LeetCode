pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
        let mut max_diff = 0;
        let mut diff_sum = 0;

        for transaction in transactions {
            let [cost, cashback] = <[_; 2]>::map(transaction.try_into().ok().unwrap(), |x| x as u32);

            let min_diff = if cashback < cost {
                diff_sum += u64::from(cost - cashback);

                cashback
            } else {
                cost
            };

            max_diff = max_diff.max(min_diff);
        }

        (diff_sum + u64::from(max_diff)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
        Self::minimum_money(transactions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
