pub mod sliding_window;

pub trait Solution {
    fn take_characters(s: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("aabaaaacaabc", 2), 8), (("a", 1), -1)];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::take_characters(s.to_string(), k), expected);
        }
    }
}
