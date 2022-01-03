pub mod brute_force;

pub trait Solution {
    fn prime_palindrome(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // https://oeis.org/A002385.
        let test_cases = [
            (1, 2),
            (2, 2),
            (3, 3),
            (4, 5),
            (5, 5),
            (6, 7),
            (7, 7),
            (8, 11),
            (9, 11),
            (10, 11),
            (11, 11),
            (12, 101),
            (13, 101),
            (98, 101),
            (99, 101),
            (100, 101),
            (101, 101),
            (102, 131),
            (130, 131),
            (131, 131),
            (85_709_140, 100_030_001),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::prime_palindrome(n), expected);
        }
    }
}
