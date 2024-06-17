pub mod stack;

pub trait Solution {
    fn remove_duplicates(s: String, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abcd", 2), "abcd"),
            (("deeedbbcccbdaa", 3), "aa"),
            (("pbbcggttciiippooaais", 2), "ps"),
        ];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::remove_duplicates(s.to_string(), k), expected);
        }
    }
}
