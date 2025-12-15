pub mod dfs;

pub trait Solution {
    fn valid_strings(n: i32) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [(3, &["010", "011", "101", "110", "111"] as &[_]), (1, &["0", "1"])];

        for (n, expected) in test_cases {
            assert_eq!(test_utilities::unstable_sorted(S::valid_strings(n)), expected);
        }
    }
}
