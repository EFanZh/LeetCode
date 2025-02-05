pub mod iterative;

pub trait Solution {
    fn equal_frequency(word: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abcc", true), ("aazz", false), ("bac", true)];

        for (word, expected) in test_cases {
            assert_eq!(S::equal_frequency(word.to_string()), expected);
        }
    }
}
