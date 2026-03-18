pub mod greedy;

pub trait Solution {
    fn find_valid_pair(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("2523533", "23"), ("221", "21"), ("22", "")];

        for (s, expected) in test_cases {
            assert_eq!(S::find_valid_pair(s.to_string()), expected);
        }
    }
}
