pub mod greedy;

pub trait Solution {
    fn remove_duplicates(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abbaca", "ca"), ("azxxzy", "ay")];

        for (s, expected) in test_cases {
            assert_eq!(S::remove_duplicates(s.to_string()), expected);
        }
    }
}
