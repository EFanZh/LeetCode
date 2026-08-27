pub mod iterative;

pub trait Solution {
    fn longest_balanced(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abbac", 4), ("zzabccy", 4), ("aba", 2)];

        for (s, expected) in test_cases {
            assert_eq!(S::longest_balanced(s.to_string()), expected);
        }
    }
}
