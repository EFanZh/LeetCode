pub mod bit_manipulation;

pub trait Solution {
    fn even_odd_bit(n: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(50, [1, 2]), (2, [0, 1])];

        for (n, expected) in test_cases {
            assert_eq!(S::even_odd_bit(n), expected);
        }
    }
}
