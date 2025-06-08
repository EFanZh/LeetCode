pub mod iterative;

pub trait Solution {
    fn minimized_string_length(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aaabc", 3), ("cbbd", 3), ("baadccab", 4)];

        for (s, expected) in test_cases {
            assert_eq!(S::minimized_string_length(s.to_string()), expected);
        }
    }
}
