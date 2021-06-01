pub mod dynamic_programming;
pub mod dynamic_programming_binary_heap;

pub trait Solution {
    fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((12, &[2, 7, 13, 19] as &[_]), 32),
            ((1, &[2, 3, 5]), 1),
            ((10, &[2, 5, 7, 11, 13, 17, 23, 29, 43, 53]), 14),
        ];

        for ((n, primes), expected) in test_cases.iter().copied() {
            assert_eq!(S::nth_super_ugly_number(n, primes.to_vec()), expected);
        }
    }
}
