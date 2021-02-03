pub mod iterative;
pub mod recursive_descent;
pub mod stack;

pub trait Solution {
    fn calculate(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("3+2*2", 7),
            (" 3/2 ", 1),
            (" 3+5 / 2 ", 5),
            ("1*2-3/4+5*6-7*8+9/10", -24),
            ("2 * 3 * 5 / 7 - 11", -7),
        ];

        for (s, expected) in test_cases.iter().copied() {
            assert_eq!(S::calculate(s.to_string()), expected);
        }
    }
}
