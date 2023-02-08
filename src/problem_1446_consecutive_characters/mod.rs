pub mod iterative;

pub trait Solution {
    fn max_power(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("leetcode", 2), ("abbcccddddeeeeedcba", 5), ("tourist", 1)];

        for (s, expected) in test_cases {
            assert_eq!(S::max_power(s.to_string()), expected);
        }
    }
}
