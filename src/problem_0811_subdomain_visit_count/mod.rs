pub mod hash_map;

pub trait Solution {
    fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &["9001 discuss.leetcode.com"] as &[_],
                &["9001 com", "9001 discuss.leetcode.com", "9001 leetcode.com"] as &[_],
            ),
            (
                &["900 google.mail.com", "50 yahoo.com", "1 intel.mail.com", "5 wiki.org"],
                &[
                    "1 intel.mail.com",
                    "5 org",
                    "5 wiki.org",
                    "50 yahoo.com",
                    "900 google.mail.com",
                    "901 mail.com",
                    "951 com",
                ],
            ),
        ];

        for (cpdomains, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::subdomain_visits(
                    cpdomains.iter().copied().map(str::to_string).collect()
                )),
                expected
            );
        }
    }
}
