pub mod brute_force;

pub trait Solution {
    fn buddy_strings(s: String, goal: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("ab", "ba"), true),
            (("ab", "ab"), false),
            (("aa", "aa"), true),
            (("aaaaaaabc", "aaaaaaacb"), true),
            (("ab", "babbb"), false),
            (("abac", "abad"), false),
            (("abcd", "badc"), false),
            (("abcd", "bacd"), true),
            (("acccccb", "bccccca"), true),
        ];

        for ((s, goal), expected) in test_cases {
            assert_eq!(S::buddy_strings(s.to_string(), goal.to_string()), expected);
        }
    }
}
