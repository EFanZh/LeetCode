pub mod generic;
pub mod specialized_1;
pub mod specialized_2;

pub trait Solution {
    fn int_to_roman(num: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = [(3, "III"), (4, "IV"), (9, "IX"), (58, "LVIII"), (1994, "MCMXCIV")];

        for (num, expected) in test_cases.iter().copied() {
            assert_eq!(S::int_to_roman(num), expected);
        }
    }
}
