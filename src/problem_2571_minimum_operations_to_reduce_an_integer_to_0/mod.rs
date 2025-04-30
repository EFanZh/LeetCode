pub mod greedy;

pub trait Solution {
    fn min_operations(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(39, 3), (54, 3)];

        for (n, expected) in test_cases {
            assert_eq!(S::min_operations(n), expected);
        }
    }
}
