pub mod dynamic_programming;

pub trait Solution {
    fn palindrome_partition(s: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("abc", 2), 1), (("aabbc", 3), 0), (("leetcode", 8), 0)];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::palindrome_partition(s.to_string(), k), expected);
        }
    }
}
