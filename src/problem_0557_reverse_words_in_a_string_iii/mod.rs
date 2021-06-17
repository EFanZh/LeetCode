pub mod iterative;

pub trait Solution {
    fn reverse_words(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("Let's take LeetCode contest", "s'teL ekat edoCteeL tsetnoc"),
            ("God Ding", "doG gniD"),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::reverse_words(s.to_string()), expected);
        }
    }
}
