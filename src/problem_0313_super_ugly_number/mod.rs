pub mod dynamic_programming;
pub mod dynamic_programming_binary_heap;

pub trait Solution {
    fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((12, &[2, 7, 13, 19] as &[_]), 32)];

        for ((n, primes), expected) in test_cases.iter().copied() {
            assert_eq!(S::nth_super_ugly_number(n, primes.to_vec()), expected);
        }
    }
}
