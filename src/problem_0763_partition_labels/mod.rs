pub mod greedy;
pub mod merge_intervals;

pub trait Solution {
    fn partition_labels(s: String) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("ababcbacadefegdehijhklij", &[7, 8, 9] as &[_])];

        for (s, expected) in test_cases.iter().copied() {
            assert_eq!(
                test_utilities::unstable_sorted(S::partition_labels(s.to_string())),
                expected
            );
        }
    }
}
