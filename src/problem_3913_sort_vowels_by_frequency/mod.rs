pub mod iterative;

pub trait Solution {
    fn sort_vowels(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("leetcode", "leetcedo"),
            ("aeiaaioooa", "aaaaoooiie"),
            ("baeiou", "baeiou"),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::sort_vowels(s.to_string()), expected);
        }
    }
}
