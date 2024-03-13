pub mod iterative;

pub trait Solution {
    fn prefix_count(words: Vec<String>, pref: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["pay", "attention", "practice", "attend"], "at"), 2),
            ((&["leetcode", "win", "loops", "success"], "code"), 0),
        ];

        for ((words, pref), expected) in test_cases {
            assert_eq!(
                S::prefix_count(words.iter().copied().map(str::to_string).collect(), pref.to_string()),
                expected,
            );
        }
    }
}
