pub mod parsing;

pub trait Solution {
    fn roman_to_int(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = [("III", 3), ("IV", 4), ("IX", 9), ("LVIII", 58), ("MCMXCIV", 1994)];

        for (num, expected) in test_cases.iter().copied() {
            assert_eq!(S::roman_to_int(num.to_owned()), expected);
        }
    }
}
