pub mod bit_manipulation;

pub trait Solution {
    fn check_if_pangram(sentence: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("thequickbrownfoxjumpsoverthelazydog", true), ("leetcode", false)];

        for (sentence, expected) in test_cases {
            assert_eq!(S::check_if_pangram(sentence.to_string()), expected);
        }
    }
}
