pub mod buckets;
pub mod even_odd;

pub trait Solution {
    fn reorganize_string(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [("aab", true), ("aaab", false)];

        for (s, is_possible) in test_cases {
            let result = S::reorganize_string(s.to_string());

            if is_possible {
                assert_eq!(
                    test_utilities::unstable_sorted(s.bytes()),
                    test_utilities::unstable_sorted(result.bytes())
                );

                assert!(result
                    .bytes()
                    .zip(result.bytes().skip(1))
                    .all(|(left, right)| left != right));
            } else {
                assert!(result.is_empty());
            }
        }
    }
}
