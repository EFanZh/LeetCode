pub mod dynamic_programming;

pub trait Solution {
    fn distinct_subseq_ii(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abc", 7), ("aba", 6), ("aaa", 3)];

        for (s, expected) in test_cases {
            assert_eq!(S::distinct_subseq_ii(s.to_string()), expected);
        }
    }
}
