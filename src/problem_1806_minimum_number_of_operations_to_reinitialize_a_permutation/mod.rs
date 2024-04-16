pub mod reversed_iteration;

pub trait Solution {
    fn reinitialize_permutation(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(2, 1), (4, 2), (6, 4), (8, 3), (10, 6)];

        for (n, expected) in test_cases {
            assert_eq!(S::reinitialize_permutation(n), expected);
        }
    }
}
