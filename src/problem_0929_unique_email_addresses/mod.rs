pub mod iterative;

pub trait Solution {
    fn num_unique_emails(emails: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    "test.email+alex@leetcode.com",
                    "test.e.mail+bob.cathy@leetcode.com",
                    "testemail+david@lee.tcode.com",
                ] as &[_],
                2,
            ),
            (&["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"], 3),
        ];

        for (emails, expected) in test_cases {
            assert_eq!(
                S::num_unique_emails(emails.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
