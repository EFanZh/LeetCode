pub mod brute_force;
pub mod sieve;

pub trait Solution {
    fn count_primes(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, 0),
            (2, 0),
            (3, 1),
            (4, 2),
            (5, 2),
            (6, 3),
            (7, 3),
            (8, 4),
            (9, 4),
            (10, 4),
        ];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(S::count_primes(n), expected);
        }
    }
}
