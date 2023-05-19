pub mod dynamic_programming;
pub mod dynamic_programming_2;

pub trait Solution {
    fn minimum_deletions(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aababbab", 2), ("bbaaaaabb", 2)];

        for (s, expected) in test_cases {
            assert_eq!(S::minimum_deletions(s.to_string()), expected);
        }
    }
}
