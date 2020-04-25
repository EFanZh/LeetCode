pub mod parsing;
pub mod parsing_2;

pub trait Solution {
    fn roman_to_int(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("III", 3), ("IV", 4), ("IX", 9), ("LVIII", 58), ("MCMXCIV", 1994)];

        for (s, expected) in test_cases.iter().copied() {
            assert_eq!(S::roman_to_int(s.to_string()), expected);
        }
    }
}
