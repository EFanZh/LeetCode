pub mod brute_force;

pub trait Solution {
    fn nearest_palindromic(n: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // https://oeis.org/A262040.
        let test_cases = [
            (1, 0),
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 4),
            (6, 5),
            (7, 6),
            (8, 7),
            (9, 8),
            (10, 9),
            (11, 9),
            (12, 11),
            (13, 11),
            (99, 101),
            (101, 99),
            (111, 101),
            (123, 121),
            (999, 1001),
            (1001, 999),
            (1111, 1001),
            (1283, 1331),
            (1213, 1221),
            (9999, 10001),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::nearest_palindromic(n.to_string()).parse::<u64>().unwrap(), expected);
        }
    }
}
