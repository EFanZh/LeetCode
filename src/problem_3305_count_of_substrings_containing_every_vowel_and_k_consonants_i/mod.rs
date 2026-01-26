pub mod sliding_window;

pub trait Solution {
    fn count_of_substrings(word: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("aeioqq", 1), 0), (("aeiou", 0), 1), (("ieaouqqieaouqq", 1), 3)];

        for ((word, k), expected) in test_cases {
            assert_eq!(S::count_of_substrings(word.to_string(), k), expected);
        }
    }
}
