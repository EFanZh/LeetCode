pub mod iterative;

pub trait Solution {
    fn restore_ip_addresses(s: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("25525511135", &["255.255.11.135", "255.255.111.35"] as &[_]),
            ("0000", &["0.0.0.0"]),
            ("010010", &["0.10.0.10", "0.100.1.0"]),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::restore_ip_addresses(s.to_string()), expected);
        }
    }
}
