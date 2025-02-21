pub mod iterative;

pub trait Solution {
    fn modify_string(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = ["?zs", "ubv?w", "j?qg??b", "??yw?ipkj?", "b?a"];

        for s in test_cases {
            let result = S::modify_string(s.to_string());

            assert_eq!(result.len(), s.len());

            assert!(
                s.bytes()
                    .zip(result.bytes())
                    .all(|(lhs, rhs)| rhs.is_ascii_lowercase() && (lhs == b'?' || lhs == rhs))
            );

            let mut prev = 0;

            for c in result.bytes() {
                assert_ne!(c, prev);
                prev = c;
            }
        }
    }
}
