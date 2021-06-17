pub mod stack;

pub trait Solution {
    fn remove_duplicate_letters(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("bcabc", "abc"), ("cbacdcbc", "acdb")];

        for (s, expected) in test_cases {
            assert_eq!(S::remove_duplicate_letters(s.to_string()), expected);
        }
    }
}
