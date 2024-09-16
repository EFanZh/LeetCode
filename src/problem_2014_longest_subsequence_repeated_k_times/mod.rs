pub mod bfs;

pub trait Solution {
    fn longest_subsequence_repeated_k(s: String, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("letsleetcode", 2), "let"), (("bb", 2), "b"), (("ab", 2), "")];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::longest_subsequence_repeated_k(s.to_string(), k), expected);
        }
    }
}
