pub mod greedy;

pub trait Solution {
    fn minimum_length(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abaacbcbb", 5), ("aa", 2)];

        for (s, expected) in test_cases {
            assert_eq!(S::minimum_length(s.to_string()), expected);
        }
    }
}
