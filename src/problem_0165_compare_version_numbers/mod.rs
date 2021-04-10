pub mod iterator;

pub trait Solution {
    fn compare_version(version1: String, version2: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("0.1", "1.1"), -1),
            (("1.0.1", "1"), 1),
            (("7.5.2.4", "7.5.3"), -1),
            (("1.01", "1.001"), 0),
            (("1.0", "1.0.0"), 0),
            (("1", "1.1"), -1),
            (("1.0", "1"), 0),
            (("1", "0"), 1),
        ];

        for ((version1, version2), expected) in test_cases.iter().copied() {
            assert_eq!(S::compare_version(version1.to_string(), version2.to_string()), expected);
        }
    }
}
