pub mod iterative;

pub trait Solution {
    fn check_divisibility(n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(99, true), (23, false)];

        for (n, expected) in test_cases {
            assert_eq!(S::check_divisibility(n), expected);
        }
    }
}
