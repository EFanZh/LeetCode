pub mod sliding_window;
pub mod sliding_window_2;

pub trait Solution {
    fn min_window(s: String, t: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("ADOBECODEBANC", "ABC"), "BANC"),
            (("a", "aa"), ""),
            (("bba", "ab"), "ba"),
            (("abc", ""), ""),
        ];

        for ((s, t), expected) in test_cases {
            assert_eq!(S::min_window(s.to_string(), t.to_string()), expected);
        }
    }
}
