pub mod iterative;

pub trait Solution {
    fn longest_str_chain(words: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["a", "b", "ba", "bca", "bda", "bdca"] as &[_], 4),
            (&["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"], 5),
            (&["abcd", "dbqca"], 1),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                S::longest_str_chain(words.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
