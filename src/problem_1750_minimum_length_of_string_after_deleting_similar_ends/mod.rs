pub mod iterative;

pub trait Solution {
    fn minimum_length(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("ca", 2),
            ("cabaabac", 0),
            ("aabccabba", 3),
            ("bbbbbbbbbbbbbbbbbbbbbbbbbbbabbbbbbbbbbbbbbbccbcbcbccbbabbb", 1),
            ("c", 1),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::minimum_length(s.to_string()), expected);
        }
    }
}
