pub mod brute_force;

pub trait Solution {
    fn convert_date_to_binary(date: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("2080-02-29", "100000100000-10-11101"),
            ("1900-01-01", "11101101100-1-1"),
        ];

        for (date, expected) in test_cases {
            assert_eq!(S::convert_date_to_binary(date.to_string()), expected);
        }
    }
}
