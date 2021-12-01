pub mod iterative;

pub trait Solution {
    fn binary_gap(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(22, 2), (5, 2), (6, 1), (8, 0), (1, 0)];

        for (n, expected) in test_cases {
            assert_eq!(S::binary_gap(n), expected);
        }
    }
}
