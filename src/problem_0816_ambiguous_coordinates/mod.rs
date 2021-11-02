pub mod brute_force;

pub trait Solution {
    fn ambiguous_coordinates(s: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("(123)", &["(1, 2.3)", "(1, 23)", "(1.2, 3)", "(12, 3)"] as &[_]),
            (
                "(0123)",
                &[
                    "(0, 1.23)",
                    "(0, 12.3)",
                    "(0, 123)",
                    "(0.1, 2.3)",
                    "(0.1, 23)",
                    "(0.12, 3)",
                ],
            ),
            ("(00011)", &["(0, 0.011)", "(0.001, 1)"]),
            ("(100)", &["(10, 0)"]),
            ("(010)", &["(0, 10)", "(0.1, 0)"]),
        ];

        for (s, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::ambiguous_coordinates(s.to_string())),
                expected
            );
        }
    }
}
