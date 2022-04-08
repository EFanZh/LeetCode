pub mod iterative;

pub trait Solution {
    fn clumsy(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 1), (2, 2), (4, 7), (5, 7), (10, 12)];

        for (n, expected) in test_cases {
            assert_eq!(S::clumsy(n), expected);
        }
    }
}
