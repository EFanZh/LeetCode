pub mod iterative;

pub trait Solution {
    fn repeated_character(s: String) -> char;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abccbaacz", 'c'), ("abcdd", 'd')];

        for (s, expected) in test_cases {
            assert_eq!(S::repeated_character(s.to_string()), expected);
        }
    }
}
