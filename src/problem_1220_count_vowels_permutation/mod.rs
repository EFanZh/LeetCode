pub mod matrix_multiplication;

pub trait Solution {
    fn count_vowel_permutation(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 5), (2, 10), (5, 68)];

        for (n, expected) in test_cases {
            assert_eq!(S::count_vowel_permutation(n), expected);
        }
    }
}
