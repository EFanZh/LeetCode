pub mod sliding_window;
pub mod sliding_window_2;

pub trait Solution {
    fn find_anagrams(s: String, p: String) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("cbaebabacd", "abc"), &[0, 6] as &[_]),
            (("abab", "ab"), &[0, 1, 2]),
            (("abc", "abcd"), &[]),
        ];

        for ((s, p), expected) in test_cases {
            assert_eq!(S::find_anagrams(s.to_string(), p.to_string()), expected);
        }
    }
}
