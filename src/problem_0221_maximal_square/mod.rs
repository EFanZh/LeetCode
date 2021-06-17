pub mod dynamic_programming;
pub mod dynamic_programming_2;

pub trait Solution {
    fn maximal_square(matrix: Vec<Vec<char>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&["10100", "10111", "11111", "10010"] as &[&str], 4)];

        for (matrix, expected) in test_cases {
            assert_eq!(
                S::maximal_square(matrix.iter().map(|row| row.chars().collect()).collect()),
                expected
            );
        }
    }
}
