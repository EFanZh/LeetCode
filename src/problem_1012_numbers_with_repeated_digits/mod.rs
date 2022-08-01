pub mod iterative;

pub trait Solution {
    fn num_dup_digits_at_most_n(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(20, 1), (100, 10), (1000, 262)];

        for (n, expected) in test_cases {
            assert_eq!(S::num_dup_digits_at_most_n(n), expected);
        }
    }
}
