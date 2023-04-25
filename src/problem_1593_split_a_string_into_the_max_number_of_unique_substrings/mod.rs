pub mod backtracking;

pub trait Solution {
    fn max_unique_split(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("ababccc", 5), ("aba", 2), ("aa", 1)];

        for (s, expected) in test_cases {
            assert_eq!(S::max_unique_split(s.to_string()), expected);
        }
    }
}
