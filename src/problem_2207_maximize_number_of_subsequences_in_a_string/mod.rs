pub mod greedy;

pub trait Solution {
    fn maximum_subsequence_count(text: String, pattern: String) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("abdcdbc", "ac"), 4), (("aabb", "ab"), 6)];

        for ((text, pattern), expected) in test_cases {
            assert_eq!(
                S::maximum_subsequence_count(text.to_string(), pattern.to_string()),
                expected,
            );
        }
    }
}
