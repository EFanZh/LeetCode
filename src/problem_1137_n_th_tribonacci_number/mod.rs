pub mod matrix_multiplication;

pub trait Solution {
    fn tribonacci(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(0, 0), (1, 1), (2, 1), (3, 2), (4, 4), (25, 1_389_537)];

        for (n, expected) in test_cases {
            assert_eq!(S::tribonacci(n), expected);
        }
    }
}
