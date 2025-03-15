pub mod greedy;

pub trait Solution {
    fn append_characters(s: String, t: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("coaching", "coding"), 4), (("abcde", "a"), 0), (("z", "abcde"), 5)];

        for ((s, t), expected) in test_cases {
            assert_eq!(S::append_characters(s.to_string(), t.to_string()), expected);
        }
    }
}
