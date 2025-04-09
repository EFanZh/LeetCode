pub mod mathematical;

pub trait Solution {
    fn alternate_digit_sum(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(521, 4), (111, 1), (886_996, 0)];

        for (n, expected) in test_cases {
            assert_eq!(S::alternate_digit_sum(n), expected);
        }
    }
}
