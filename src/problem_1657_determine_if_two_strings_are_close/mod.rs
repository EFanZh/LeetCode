pub mod iterative;

pub trait Solution {
    fn close_strings(word1: String, word2: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abc", "bca"), true),
            (("a", "aa"), false),
            (("cabbba", "abbccc"), true),
            (("uau", "ssx"), false),
        ];

        for ((word1, word2), expected) in test_cases {
            assert_eq!(S::close_strings(word1.to_string(), word2.to_string()), expected);
        }
    }
}
