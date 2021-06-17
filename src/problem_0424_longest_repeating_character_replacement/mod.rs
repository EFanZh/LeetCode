pub mod sliding_window;
pub mod sliding_window_2;

pub trait Solution {
    fn character_replacement(s: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("ABAB", 2), 4), (("AABABBA", 1), 4), (("ABCDEFG", 6), 7)];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::character_replacement(s.to_string(), k), expected);
        }
    }
}
