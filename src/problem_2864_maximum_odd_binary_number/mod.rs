pub mod greedy;

pub trait Solution {
    fn maximum_odd_binary_number(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("010", "001"), ("0101", "1001")];

        for (s, expected) in test_cases {
            assert_eq!(S::maximum_odd_binary_number(s.to_string()), expected);
        }
    }
}
