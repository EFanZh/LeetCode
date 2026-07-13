pub mod memoized_dynamic_programming;

pub trait Solution {
    fn valid_digit(n: i32, x: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((101, 0), true), ((232, 2), false), ((5, 1), false)];

        for ((n, x), expected) in test_cases {
            assert_eq!(S::valid_digit(n, x), expected);
        }
    }
}
