pub mod iterative;

pub trait Solution {
    fn largest_palindrome(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // https://oeis.org/A327897 % 1337.
        let test_cases = [
            (1, 9),
            (2, 987),
            (3, 123),
            (4, 597),
            (5, 677),
            (6, 1218),
            (7, 877),
            (8, 475),
            // (9, 1226),
        ];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(S::largest_palindrome(n), expected);
        }
    }
}
