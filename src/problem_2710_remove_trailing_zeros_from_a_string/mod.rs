pub mod cheating;

pub trait Solution {
    fn remove_trailing_zeros(num: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("51230100", "512301"), ("123", "123")];

        for (num, expected) in test_cases {
            assert_eq!(S::remove_trailing_zeros(num.to_string()), expected);
        }
    }
}
