pub mod manual_reversed_iteration;
pub mod reversed_iteration;

pub trait Solution {
    fn length_of_last_word(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("Hello World", 5), (" ", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::length_of_last_word(s.to_string()), expected);
        }
    }
}
