pub mod count_leading_zeros;
pub mod xor_with_shifting;

pub trait Solution {
    fn has_alternating_bits(n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(3, false), (5, true), (7, false), (10, true), (11, false)];

        for (n, expected) in test_cases {
            assert_eq!(S::has_alternating_bits(n), expected);
        }
    }
}
