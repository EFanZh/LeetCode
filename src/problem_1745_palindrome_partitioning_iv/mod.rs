pub mod dynamic_programming;

pub trait Solution {
    fn check_partitioning(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("abcbdd", true),
            ("bcbddxy", false),
            ("babbaaaaab", false),
            ("aaa", true),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::check_partitioning(s.to_string()), expected);
        }
    }
}
