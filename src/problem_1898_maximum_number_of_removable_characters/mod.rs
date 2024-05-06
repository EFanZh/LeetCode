pub mod binary_search;

pub trait Solution {
    fn maximum_removals(s: String, p: String, removable: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abcacb", "ab", &[3, 1, 0] as &[_]), 2),
            (("abcbddddd", "abcd", &[3, 2, 1, 4, 5, 6]), 1),
            (("abcab", "abc", &[0, 1, 2, 3, 4]), 0),
        ];

        for ((s, p, removable), expected) in test_cases {
            assert_eq!(
                S::maximum_removals(s.to_string(), p.to_string(), removable.to_vec()),
                expected,
            );
        }
    }
}
