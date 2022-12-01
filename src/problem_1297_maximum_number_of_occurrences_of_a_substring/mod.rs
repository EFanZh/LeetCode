pub mod sliding_window;

pub trait Solution {
    fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("aababcaab", 2, 3, 4), 2),
            (("aaaa", 1, 3, 3), 2),
            (("abcde", 2, 3, 3), 0),
        ];

        for ((s, max_letters, min_size, max_size), expected) in test_cases {
            assert_eq!(S::max_freq(s.to_string(), max_letters, min_size, max_size), expected);
        }
    }
}
