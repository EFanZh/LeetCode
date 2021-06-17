pub mod find_repeated_pattern;

pub trait Solution {
    fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("acb", 4, "ab", 2), 2),
            (("aaa", 3, "aa", 1), 4),
            (("baba", 11, "baab", 1), 7),
            (("acb", 1, "acb", 1), 1),
        ];

        for ((s1, n1, s2, n2), expected) in test_cases {
            assert_eq!(S::get_max_repetitions(s1.to_string(), n1, s2.to_string(), n2), expected);
        }
    }
}
