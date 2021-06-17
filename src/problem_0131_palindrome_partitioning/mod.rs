pub mod find_all_palindromes;

pub trait Solution {
    fn partition(s: String) -> Vec<Vec<String>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("aab", &[&["a", "a", "b"] as &[_], &["aa", "b"]] as &[&[_]]),
            ("a", &[&["a"]]),
            ("efe", &[&["e", "f", "e"], &["efe"]]),
        ];

        for (s, expected) in test_cases {
            assert_eq!(test_utilities::unstable_sorted(S::partition(s.to_string())), expected);
        }
    }
}
