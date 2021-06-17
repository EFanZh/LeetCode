pub mod brute_force;

pub trait Solution {
    fn convert(s: String, num_rows: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("PAYPALISHIRING", 3), "PAHNAPLSIIGYIR"),
            (("PAYPALISHIRING", 4), "PINALSIGYAHRPI"),
            (("", 1), ""),
        ];

        for ((s, num_rows), expected) in test_cases {
            assert_eq!(S::convert(s.to_string(), num_rows), expected);
        }
    }
}
