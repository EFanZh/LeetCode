pub mod dynamic_programming;
pub mod iterative;

pub trait Solution {
    fn unique_letter_string(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("ABC", 10), ("ABA", 8), ("LEETCODE", 92)];

        for (s, expected) in test_cases {
            assert_eq!(S::unique_letter_string(s.to_string()), expected);
        }
    }
}
