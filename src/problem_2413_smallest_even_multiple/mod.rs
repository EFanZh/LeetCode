pub mod bit_manipulation;

pub trait Solution {
    fn smallest_even_multiple(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(5, 10), (6, 6)];

        for (n, expected) in test_cases {
            assert_eq!(S::smallest_even_multiple(n), expected);
        }
    }
}
