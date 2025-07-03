pub mod greedy;

pub trait Solution {
    fn account_balance_after_purchase(purchase_amount: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(9, 90), (15, 80), (10, 90)];

        for (purchase_amount, expected) in test_cases {
            assert_eq!(S::account_balance_after_purchase(purchase_amount), expected);
        }
    }
}
