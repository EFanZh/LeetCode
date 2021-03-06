pub mod iterative_dfs;

pub trait Solution {
    fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [(
            &[
                &["John", "johnsmith@mail.com", "john00@mail.com"] as &[_],
                &["John", "johnnybravo@mail.com"],
                &["John", "johnsmith@mail.com", "john_newyork@mail.com"],
                &["Mary", "mary@mail.com"],
            ],
            &[
                &["John", "john00@mail.com", "john_newyork@mail.com", "johnsmith@mail.com"] as &[_],
                &["John", "johnnybravo@mail.com"],
                &["Mary", "mary@mail.com"],
            ],
        )];

        for (accounts, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::accounts_merge(
                    accounts
                        .iter()
                        .map(|account| account.iter().copied().map(str::to_string).collect())
                        .collect()
                )),
                expected
            );
        }
    }
}
