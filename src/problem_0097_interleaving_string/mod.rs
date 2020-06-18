pub mod dynamic_programming;

pub trait Solution {
    fn is_interleave(s1: String, s2: String, s3: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("aabcc", "dbbca", "aadbbcbcac"), true),
            (("aabcc", "dbbca", "aadbbbaccc"), false),
            (("", "", "a"), false),
            (("a", "", "c"), false),
        ];

        for ((s1, s2, s3), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::is_interleave(s1.to_string(), s2.to_string(), s3.to_string()),
                expected
            );
        }
    }
}
