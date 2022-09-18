pub mod greedy_with_rolling_hash;

pub trait Solution {
    fn longest_decomposition(text: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("ghiabcdefhelloadamhelloabcdefghi", 7),
            ("merchant", 1),
            ("antaprezatepzapreanta", 11),
            ("elvtoelvto", 2),
        ];

        for (text, expected) in test_cases {
            assert_eq!(S::longest_decomposition(text.to_string()), expected);
        }
    }
}
