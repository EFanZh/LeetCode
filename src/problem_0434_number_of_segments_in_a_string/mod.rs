pub mod cheating;
pub mod iterative;

pub trait Solution {
    fn count_segments(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("Hello, my name is John", 5),
            ("Hello", 1),
            ("love live! mu'sic forever", 4),
            ("", 0),
        ];

        for (s, expected) in test_cases.iter().copied() {
            assert_eq!(S::count_segments(s.to_string()), expected);
        }
    }
}
