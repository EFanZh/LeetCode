pub mod precomputed;

pub trait Solution {
    fn three_fibonacci_sum(n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(2, true), (8, false), (16, true)];

        for (n, expected) in test_cases {
            assert_eq!(S::three_fibonacci_sum(n), expected);
        }
    }
}
