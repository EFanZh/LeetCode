pub mod mathematical;

pub trait Solution {
    fn count_vowel_strings(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 5), (2, 15), (33, 66045)];

        for (n, expected) in test_cases {
            assert_eq!(S::count_vowel_strings(n), expected,);
        }
    }
}
