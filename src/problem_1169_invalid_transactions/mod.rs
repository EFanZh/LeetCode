pub mod sliding_window;

pub trait Solution {
    fn invalid_transactions(transactions: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &["alice,20,800,mtv", "alice,50,100,beijing"] as &[_],
                &["alice,20,800,mtv", "alice,50,100,beijing"] as &[_],
            ),
            (&["alice,20,800,mtv", "alice,50,1200,mtv"], &["alice,50,1200,mtv"]),
            (&["alice,20,800,mtv", "bob,50,1200,mtv"], &["bob,50,1200,mtv"]),
            (
                &[
                    "alex,676,260,bangkok",
                    "bob,656,1366,bangkok",
                    "alex,393,616,bangkok",
                    "bob,820,990,amsterdam",
                    "alex,596,1390,amsterdam",
                ],
                &["alex,596,1390,amsterdam", "bob,656,1366,bangkok"],
            ),
            (
                &[
                    "alice,20,800,mtv",
                    "bob,50,1200,mtv",
                    "alice,20,800,mtv",
                    "alice,50,1200,mtv",
                    "alice,20,800,mtv",
                    "alice,50,100,beijing",
                ],
                &[
                    "alice,20,800,mtv",
                    "alice,20,800,mtv",
                    "alice,20,800,mtv",
                    "alice,50,100,beijing",
                    "alice,50,1200,mtv",
                    "bob,50,1200,mtv",
                ],
            ),
        ];

        for (transactions, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::invalid_transactions(
                    transactions.iter().copied().map(str::to_string).collect()
                )),
                expected,
            );
        }
    }
}
