pub mod brute_force;
pub mod precomputed;

pub trait Solution {
    fn is_fascinating(n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (100, false),
            (101, false),
            (102, false),
            (103, false),
            (192, true),
            (267, false),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::is_fascinating(n), expected);
        }
    }
}
