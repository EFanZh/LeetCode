pub mod implicit_carry;
pub mod implicit_carry_2;
pub mod naive;

pub trait Solution {
    fn add_strings(num1: String, num2: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("0", "0"), "0"),
            (("999", "1"), "1000"),
            (("1", "999"), "1000"),
            (("899", "3"), "902"),
            (("999", "999"), "1998"),
            (("11", "123"), "134"),
            (("237", "284"), "521"),
        ];

        for ((num1, num2), expected) in test_cases.iter().copied() {
            assert_eq!(S::add_strings(num1.to_string(), num2.to_string()), expected);
        }
    }
}
