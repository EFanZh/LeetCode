pub mod dynamic_programming;
pub mod matrix_multiplication;

pub trait Solution {
    fn check_record(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 3), (2, 8), (10101, 183_236_316)];

        for (n, expected) in test_cases {
            assert_eq!(S::check_record(n), expected);
        }
    }
}
