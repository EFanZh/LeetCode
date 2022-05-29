pub mod dynamic_programming;

pub trait Solution {
    fn shortest_common_supersequence(str1: String, str2: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abac", "cab"), "cabac"),
            (("aaaaaaaa", "aaaaaaaa"), "aaaaaaaa"),
            (("bbbaaaba", "bbababbb"), "bbbaaababbb"),
        ];

        for ((str1, str2), expected) in test_cases {
            assert_eq!(
                S::shortest_common_supersequence(str1.to_string(), str2.to_string()),
                expected
            );
        }
    }
}
