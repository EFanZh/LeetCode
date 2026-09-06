pub mod iterative;

pub trait Solution {
    fn is_palindromic(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("f", true), ("leet", false), ("a", false)];

        for (s, expected) in test_cases {
            assert_eq!(S::is_palindromic(s.to_string()), expected);
        }
    }
}
