pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        let purchase_amount = purchase_amount as u32;

        ((104 - purchase_amount) / 10 * 10) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        Self::account_balance_after_purchase(purchase_amount)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
