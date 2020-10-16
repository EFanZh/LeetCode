pub mod bit_manipulation;
pub mod division;
pub mod enumeration;

pub trait Solution {
    fn is_power_of_two(n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, true), (16, true), (218, false)];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(S::is_power_of_two(n), expected);
        }
    }
}
