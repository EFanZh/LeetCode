pub mod sieve_of_eratosthenes;

pub trait Solution {
    fn find_prime_pairs(n: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(10, &[[3, 7], [5, 5]] as &[_]), (2, &[])];

        for (n, expected) in test_cases {
            assert_eq!(S::find_prime_pairs(n), expected);
        }
    }
}
