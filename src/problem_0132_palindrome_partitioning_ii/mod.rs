pub mod dynamic_programming;

pub trait Solution {
    fn min_cut(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aab", 1), ("a", 0), ("ab", 1), ("aab", 1), ("efe", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::min_cut(s.to_string()), expected);
        }
    }
}
