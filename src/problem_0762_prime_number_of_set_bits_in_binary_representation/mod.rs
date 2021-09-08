pub mod brute_force;
pub mod combinations;

pub trait Solution {
    fn count_prime_set_bits(left: i32, right: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((6, 10), 4),
            ((10, 15), 5),
            ((842, 888), 23),
            ((999_000, 1_000_000), 272),
        ];

        for ((left, right), expected) in test_cases {
            assert_eq!(S::count_prime_set_bits(left, right), expected);
        }
    }
}
