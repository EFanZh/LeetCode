pub mod iterative;

pub trait Solution {
    fn remove_digit(number: String, digit: char) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("123", '3'), "12"),
            (("1231", '1'), "231"),
            (("551", '5'), "51"),
            (("73197", '7'), "7319"),
            (("2464", '6'), "244"),
        ];

        for ((number, digit), expected) in test_cases {
            assert_eq!(S::remove_digit(number.to_string(), digit), expected);
        }
    }
}
