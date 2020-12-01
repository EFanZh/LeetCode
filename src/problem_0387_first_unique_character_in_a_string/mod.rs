pub mod iterative;

pub trait Solution {
    fn first_uniq_char(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("leetcode", 0), ("loveleetcode", 2)];

        for (s, expected) in test_cases.iter().copied() {
            assert_eq!(S::first_uniq_char(s.to_string()), expected);
        }
    }
}
