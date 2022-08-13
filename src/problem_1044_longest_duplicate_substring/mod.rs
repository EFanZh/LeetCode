pub mod binary_search_and_rolling_hash;

pub trait Solution {
    fn longest_dup_substring(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("banana", "ana"),
            ("abcd", ""),
            ("nnpxouomcofdjuujloanjimymadkuepightrfodmauhrsy", "ma"),
            ("zwuceyxigoigzwuceyxigoi", "zwuceyxigoi"),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::longest_dup_substring(s.to_string()), expected);
        }
    }
}
