pub mod hash_set;

pub trait Solution {
    fn maximum_number_of_string_pairs(words: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["cd", "ac", "dc", "ca", "zz"] as &[_], 2),
            (&["ab", "ba", "cc"], 1),
            (&["aa", "ab"], 0),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                S::maximum_number_of_string_pairs(words.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
