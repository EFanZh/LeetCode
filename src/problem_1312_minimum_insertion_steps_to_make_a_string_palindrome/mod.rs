pub mod dynamic_programming;
pub mod memoized_dynamic_programming;

pub trait Solution {
    fn min_insertions(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("zzazz", 0), ("mbadm", 2), ("leetcode", 5)];

        for (s, expected) in test_cases {
            assert_eq!(S::min_insertions(s.to_string()), expected);
        }
    }
}
