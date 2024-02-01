pub mod iterative;

pub trait Solution {
    fn first_palindrome(words: Vec<String>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["abc", "car", "ada", "racecar", "cool"] as &[_], "ada"),
            (&["notapalindrome", "racecar"], "racecar"),
            (&["def", "ghi"], ""),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                S::first_palindrome(words.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
