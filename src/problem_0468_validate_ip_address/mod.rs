pub mod check_one_by_one;

pub trait Solution {
    fn valid_ip_address(ip: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("172.16.254.1", "IPv4"),
            ("2001:0db8:85a3:0:0:8A2E:0370:7334", "IPv6"),
            ("256.256.256.256", "Neither"),
            ("2001:0db8:85a3:0:0:8A2E:0370:7334:", "Neither"),
            ("1e1.4.5.6", "Neither"),
            ("2001:db8:85a3:0::8a2E:0370:7334", "Neither"),
            ("01.01.01.01", "Neither"),
            ("234.234.234.224", "IPv4"),
        ];

        for (ip, expected) in test_cases {
            assert_eq!(S::valid_ip_address(ip.to_string()), expected);
        }
    }
}
