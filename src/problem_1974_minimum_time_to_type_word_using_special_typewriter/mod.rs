pub mod iterative;

pub trait Solution {
    fn min_time_to_type(word: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abc", 5), ("bza", 7), ("zjpc", 34)];

        for (word, expected) in test_cases {
            assert_eq!(S::min_time_to_type(word.to_string()), expected);
        }
    }
}
