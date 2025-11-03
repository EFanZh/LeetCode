pub mod bit_manipulation;

pub trait Solution {
    fn number_of_special_chars(word: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aaAbcBC", 3), ("abc", 0), ("AbBCab", 0)];

        for (word, expected) in test_cases {
            assert_eq!(S::number_of_special_chars(word.to_string()), expected);
        }
    }
}
