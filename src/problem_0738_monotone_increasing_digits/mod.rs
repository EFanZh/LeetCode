pub mod iterative;

pub trait Solution {
    fn monotone_increasing_digits(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (8, 8),
            (9, 9),
            (10, 9),
            (11, 11),
            (12, 12),
            (13, 13),
            (97, 89),
            (98, 89),
            (99, 99),
            (100, 99),
            (101, 99),
            (332, 299),
            (1234, 1234),
            (23_577_423, 23_569_999),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::monotone_increasing_digits(n), expected);
        }
    }
}
