pub mod prefix_sums;

pub trait Solution {
    fn sum_of_primes_in_range(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(13, 132), (10, 17), (8, 0)];

        for (n, expected) in test_cases {
            assert_eq!(S::sum_of_primes_in_range(n), expected);
        }
    }
}
