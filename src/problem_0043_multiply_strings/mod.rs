pub mod naive;

pub trait Solution {
    fn multiply(num1: String, num2: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("2", "3"), "6"),
            (("123", "456"), "56088"),
            (("5", "12"), "60"),
            (("999", "999"), "998001"),
            (("9133", "0"), "0"),
        ];

        for ((num1, num2), expected) in test_cases {
            assert_eq!(S::multiply(num1.to_string(), num2.to_string()), expected);
        }
    }
}
