pub mod iterative;

pub trait Solution {
    fn trim_trailing_vowels(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("idea", "id"), ("day", "day"), ("aeiou", "")];

        for (s, expected) in test_cases {
            assert_eq!(S::trim_trailing_vowels(s.to_string()), expected);
        }
    }
}
