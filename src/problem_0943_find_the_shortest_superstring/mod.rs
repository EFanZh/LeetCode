pub mod kmp_and_held_karp;

pub trait Solution {
    fn shortest_superstring(words: Vec<String>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["alex", "loves", "leetcode"] as &[_], 17),
            (&["catg", "ctaagt", "gcta", "ttca", "atgcatc"], 16),
            (
                &[
                    "bbabbaa",
                    "aaaaaa",
                    "aaabab",
                    "baaabba",
                    "bbbaabbbabaab",
                    "abbabaaabbbabaaabbbb",
                    "ababbba",
                ],
                46,
            ),
        ];

        for (words, expected_length) in test_cases {
            let result = S::shortest_superstring(words.iter().copied().map(str::to_string).collect());

            assert_eq!(result.len(), expected_length);

            for word in words {
                assert!(result.contains(word));
            }
        }
    }
}
