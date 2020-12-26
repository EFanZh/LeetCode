pub mod mathematical;

pub trait Solution {
    fn integer_break(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (2, 1),
            (3, 2),
            // From now on, https://oeis.org/A000792.
            (4, 4),
            (5, 6),
            (6, 9),
            (7, 12),
            (8, 18),
            (9, 27),
            (10, 36),
            (11, 54),
            (12, 81),
            (13, 108),
            (14, 162),
            (15, 243),
            (16, 324),
            (17, 486),
        ];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(S::integer_break(n), expected);
        }
    }
}
