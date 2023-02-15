pub mod iterative;

pub trait Solution {
    fn is_prefix_of_word(sentence: String, search_word: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("i love eating burger", "burg"), 4),
            (("this problem is an easy problem", "pro"), 2),
            (("i am tired", "you"), -1),
        ];

        for ((sentence, search_word), expected) in test_cases {
            assert_eq!(
                S::is_prefix_of_word(sentence.to_string(), search_word.to_string()),
                expected,
            );
        }
    }
}
