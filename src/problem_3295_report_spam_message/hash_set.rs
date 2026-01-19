pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn report_spam(message: Vec<String>, banned_words: Vec<String>) -> bool {
        let banned_words = banned_words.iter().map(String::as_str).collect::<HashSet<_>>();
        let mut message_iter = message.iter().map(String::as_str);
        let mut check = || message_iter.any(|message| banned_words.contains(message));

        check() && check()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn report_spam(message: Vec<String>, banned_words: Vec<String>) -> bool {
        Self::report_spam(message, banned_words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
