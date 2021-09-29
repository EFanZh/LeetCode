pub mod find_in_repeated_string;

pub trait Solution {
    fn rotate_string(s: String, goal: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abcde", "cdeab"), true),
            (("abcde", "abced"), false),
            (("aa", "a"), false),
        ];

        for ((s, goal), expected) in test_cases {
            assert_eq!(S::rotate_string(s.to_string(), goal.to_string()), expected);
        }
    }
}
