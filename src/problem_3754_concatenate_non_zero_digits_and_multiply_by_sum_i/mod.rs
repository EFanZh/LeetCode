pub mod iterative;

pub trait Solution {
    fn sum_and_multiply(n: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(10_203_004, 12340), (1000, 1)];

        for (n, expected) in test_cases {
            assert_eq!(S::sum_and_multiply(n), expected);
        }
    }
}
