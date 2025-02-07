pub mod gcd;

pub trait Solution {
    fn common_factors(a: i32, b: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((12, 6), 4), ((25, 30), 2), ((43, 945), 1)];

        for ((a, b), expected) in test_cases {
            assert_eq!(S::common_factors(a, b), expected);
        }
    }
}
