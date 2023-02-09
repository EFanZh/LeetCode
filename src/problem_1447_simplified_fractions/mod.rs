pub mod brute_force;

pub trait Solution {
    fn simplified_fractions(n: i32) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (2, &["1/2"] as &[_]),
            (3, &["1/2", "1/3", "2/3"]),
            (4, &["1/2", "1/3", "1/4", "2/3", "3/4"]),
        ];

        for (n, expected) in test_cases {
            assert_eq!(test_utilities::unstable_sorted(S::simplified_fractions(n)), expected);
        }
    }
}
