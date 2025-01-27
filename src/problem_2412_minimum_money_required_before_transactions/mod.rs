pub mod greedy;

pub trait Solution {
    fn minimum_money(transactions: Vec<Vec<i32>>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[[2, 1], [5, 0], [4, 2]] as &[_], 10), (&[[3, 0], [0, 3]], 3)];

        for (transactions, expected) in test_cases {
            assert_eq!(S::minimum_money(transactions.iter().map(Vec::from).collect()), expected);
        }
    }
}
