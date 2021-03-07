#![allow(non_snake_case)]

pub mod binary_search;

pub trait Solution {
    fn new(bad: i32) -> Self;
    fn isBadVersion(&self, version: i32) -> bool;
    fn first_bad_version(&self, n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((5, 4), 4), ((1, 1), 1)];

        for ((n, bad), expected) in test_cases.iter().copied() {
            assert_eq!(S::new(bad).first_bad_version(n), expected);
        }
    }
}
