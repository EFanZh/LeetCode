pub mod iterative;

pub trait Solution {
    fn vowel_consonant_score(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("cooear", 2), ("axeyizou", 1), ("au 123", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::vowel_consonant_score(s.to_string()), expected);
        }
    }
}
