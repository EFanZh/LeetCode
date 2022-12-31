pub mod iterative;

pub trait Solution {
    fn longest_prefix(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("level", "l"), ("ababab", "abab"), ("babbb", "b")];

        for (s, expected) in test_cases {
            assert_eq!(S::longest_prefix(s.to_string()), expected);
        }
    }
}
