pub mod bit_manipulation;

pub trait Solution {
    fn smallest_number(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(5, 7), (10, 15), (3, 3)];

        for (n, expected) in test_cases {
            assert_eq!(S::smallest_number(n), expected);
        }
    }
}
