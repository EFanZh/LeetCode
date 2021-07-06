pub mod dynamic_programming;

pub trait Solution {
    fn strange_printer(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aaabbb", 2), ("aba", 2)];

        for (s, expected) in test_cases {
            assert_eq!(S::strange_printer(s.to_string()), expected);
        }
    }
}
