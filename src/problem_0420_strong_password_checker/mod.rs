pub mod greedy;

pub trait Solution {
    fn strong_password_checker(password: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("a", 5),
            ("aA1", 3),
            ("1337C0d3", 0),
            ("hoAISJDBVWD09232UHJEPODKNLADU1", 10),
            ("aaa123", 1),
            ("aaaaAAAAAA000000123456", 5),
            ("FFFFFFFFFFFFFFF11111111111111111111AAA", 23),
            ("1111111111", 3),
            ("A1234567890aaabbbbccccc", 4),
            ("1234567890123456Baaaa", 2),
        ];

        for (password, expected) in test_cases.iter().copied() {
            assert_eq!(S::strong_password_checker(password.to_string()), expected);
        }
    }
}
