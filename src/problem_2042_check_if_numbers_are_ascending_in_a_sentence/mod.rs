pub mod iterative;

pub trait Solution {
    fn are_numbers_ascending(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("1 box has 3 blue 4 red 6 green and 12 yellow marbles", true),
            ("hello world 5 x 5", false),
            (
                "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s",
                false,
            ),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::are_numbers_ascending(s.to_string()), expected);
        }
    }
}
