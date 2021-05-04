pub mod brute_force;
pub mod brute_force_unsafe;
pub mod reverse_half;

pub trait Solution {
    fn is_palindrome(x: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (121, true),
            (-121, false),
            (10, false),
            (1_234_567_899, false),
            (0, true),
        ];

        for (x, expected) in test_cases.iter().copied() {
            assert_eq!(S::is_palindrome(x), expected);
        }
    }
}
