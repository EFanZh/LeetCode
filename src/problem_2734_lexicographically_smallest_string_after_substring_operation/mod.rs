pub mod greedy;

pub trait Solution {
    fn smallest_string(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("cbabc", "baabc"),
            ("aa", "az"),
            ("acbbc", "abaab"),
            ("leetcode", "kddsbncd"),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::smallest_string(s.to_string()), expected);
        }
    }
}
