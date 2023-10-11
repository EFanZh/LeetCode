pub mod greedy;

pub trait Solution {
    fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("acb", "cba", "cdb"), true),
            (("aaa", "a", "aab"), false),
            (("aaa", "a", "aaaa"), true),
        ];

        for ((first_word, second_word, target_word), expected) in test_cases {
            assert_eq!(
                S::is_sum_equal(first_word.to_string(), second_word.to_string(), target_word.to_string()),
                expected,
            );
        }
    }
}
