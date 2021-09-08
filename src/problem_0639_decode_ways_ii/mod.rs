pub mod dynamic_programming;

pub trait Solution {
    fn num_decodings(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("1*", 18),
            ("1*72*", 285),
            ("2*", 15),
            ("3*", 9),
            ("*", 9),
            ("*0**0", 36),
            ("*1", 11),
            ("*10*1", 99),
            ("*1*1*0", 404),
            ("204", 1),
            ("2839", 1),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::num_decodings(s.to_string()), expected);
        }
    }
}
