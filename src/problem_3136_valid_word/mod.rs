pub mod iterative;

pub trait Solution {
    fn is_valid(word: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("234Adas", true), ("b3", false), ("a3$e", false), ("aya", true)];

        for (word, expected) in test_cases {
            assert_eq!(S::is_valid(word.to_string()), expected);
        }
    }
}
