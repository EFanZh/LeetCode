pub mod bit_manipulation;

pub trait Solution {
    fn consecutive_set_bits(n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(6, true), (5, false), (45, true)];

        for (n, expected) in test_cases {
            assert_eq!(S::consecutive_set_bits(n), expected);
        }
    }
}
