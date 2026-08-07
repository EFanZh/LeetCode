pub mod recursive;

pub trait Solution {
    fn generate_valid_strings(n: i32, k: i32) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 1), &["000", "010", "100"] as &[_]), ((1, 0), &["0", "1"])];

        for ((n, k), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::generate_valid_strings(n, k)),
                expected,
            );
        }
    }
}
