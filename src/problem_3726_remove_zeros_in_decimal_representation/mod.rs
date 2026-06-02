pub mod iterative;

pub trait Solution {
    fn remove_zeros(num: i64) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1_020_030, 123), (1, 1)];

        for (num, expected) in test_cases {
            assert_eq!(S::remove_zeros(num), expected);
        }
    }
}
