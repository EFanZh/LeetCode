pub mod sliding_window;

pub trait Solution {
    fn number_of_substrings(s: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("abacb", 2), 4), (("abcde", 1), 15)];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::number_of_substrings(s.to_string(), k), expected);
        }
    }
}
