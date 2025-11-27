pub mod iterative;

pub trait Solution {
    fn clear_digits(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abc", "abc"), ("cb34", "")];

        for (s, expected) in test_cases {
            assert_eq!(S::clear_digits(s.to_string()), expected);
        }
    }
}
