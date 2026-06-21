pub mod iterative;

pub trait Solution {
    fn largest_even(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("1112", "1112"), ("221", "22"), ("1", "")];

        for (s, expected) in test_cases {
            assert_eq!(S::largest_even(s.to_string()), expected);
        }
    }
}
