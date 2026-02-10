pub mod dynamic_programming;
pub mod matrix_multiplication;

pub trait Solution {
    fn length_after_transformations(s: String, t: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("abcyy", 2), 7), (("azbk", 1), 5)];

        for ((s, t), expected) in test_cases {
            assert_eq!(S::length_after_transformations(s.to_string(), t), expected);
        }
    }
}
