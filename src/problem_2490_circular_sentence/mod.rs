pub mod iterative;

pub trait Solution {
    fn is_circular_sentence(sentence: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("leetcode exercises sound delightful", true),
            ("eetcode", true),
            ("Leetcode is cool", false),
            ("a", true),
        ];

        for (sentence, expected) in test_cases {
            assert_eq!(S::is_circular_sentence(sentence.to_string()), expected);
        }
    }
}
