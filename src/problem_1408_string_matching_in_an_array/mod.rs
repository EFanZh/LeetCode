pub mod brute_force;

pub trait Solution {
    fn string_matching(words: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["mass", "as", "hero", "superhero"] as &[_], &["as", "hero"] as &[_]),
            (&["leetcode", "et", "code"], &["code", "et"]),
            (&["blue", "green", "bu"], &[]),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::string_matching(
                    words.iter().copied().map(str::to_string).collect()
                )),
                expected,
            );
        }
    }
}
