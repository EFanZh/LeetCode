pub mod hash_set;

pub trait Solution {
    fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    &["leetcode", "google", "facebook"] as &[_],
                    &["google", "microsoft"],
                    &["google", "facebook"],
                    &["google"],
                    &["amazon"],
                ] as &[&[_]],
                &[0, 1, 4] as &[_],
            ),
            (
                &[
                    &["leetcode", "google", "facebook"],
                    &["leetcode", "amazon"],
                    &["facebook", "google"],
                ],
                &[0, 1],
            ),
            (&[&["leetcode"], &["google"], &["facebook"], &["amazon"]], &[0, 1, 2, 3]),
        ];

        for (favorite_companies, expected) in test_cases {
            assert_eq!(
                S::people_indexes(
                    favorite_companies
                        .iter()
                        .map(|favorite_companies| { favorite_companies.iter().copied().map(str::to_string).collect() })
                        .collect()
                ),
                expected,
            );
        }
    }
}
