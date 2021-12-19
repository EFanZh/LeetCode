pub mod hash_map;

pub trait Solution {
    fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                ("this apple is sweet", "this apple is sour"),
                &["sour", "sweet"] as &[_],
            ),
            (("apple apple", "banana"), &["banana"]),
        ];

        for ((s1, s2), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::uncommon_from_sentences(s1.to_string(), s2.to_string())),
                expected
            );
        }
    }
}
