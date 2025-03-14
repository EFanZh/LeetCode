pub mod dynamic_programming;

pub trait Solution {
    fn count_palindromes(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("103301", 2), ("0000000", 21), ("9999900000", 2)];

        for (s, expected) in test_cases {
            assert_eq!(S::count_palindromes(s.to_string()), expected);
        }
    }
}
