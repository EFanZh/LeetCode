pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .into_iter()
            .map(|account| account.iter().sum::<i32>() as u32)
            .max()
            .unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        Self::maximum_wealth(accounts)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
