pub mod iterative;

pub trait Solution {
    fn large_group_positions(s: String) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("abbxxxxzzy", &[[3, 6]] as &[_]),
            ("abc", &[]),
            ("abcdddeeeeaabbbcd", &[[3, 5], [6, 9], [12, 14]]),
            ("aba", &[]),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::large_group_positions(s.to_string()), expected);
        }
    }
}
