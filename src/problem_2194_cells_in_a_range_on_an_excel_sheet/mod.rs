pub mod iterative;

pub trait Solution {
    fn cells_in_range(s: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("K1:L2", &["K1", "K2", "L1", "L2"] as &[_]),
            ("A1:F1", &["A1", "B1", "C1", "D1", "E1", "F1"]),
        ];

        for (s, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::cells_in_range(s.to_string())),
                expected,
            );
        }
    }
}
