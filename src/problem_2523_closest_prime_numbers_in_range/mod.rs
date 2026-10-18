pub mod sieve_of_eratosthenes;

pub trait Solution {
    fn closest_primes(left: i32, right: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((10, 19), [11, 13]), ((4, 6), [-1, -1]), ((19, 31), [29, 31])];

        for ((left, right), expected) in test_cases {
            assert_eq!(S::closest_primes(left, right), expected);
        }
    }
}
