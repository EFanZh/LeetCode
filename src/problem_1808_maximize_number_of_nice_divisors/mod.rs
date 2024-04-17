pub mod mathematical;

pub trait Solution {
    fn max_nice_divisors(prime_factors: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // From now on, https://oeis.org/A000792.
        let test_cases = [
            (1, 1),
            (2, 2),
            (3, 3),
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
            (73, 572_712_676),
        ];

        for (prime_factors, expected) in test_cases {
            assert_eq!(S::max_nice_divisors(prime_factors), expected);
        }
    }
}
