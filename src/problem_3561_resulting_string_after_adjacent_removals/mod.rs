pub mod stack;

pub trait Solution {
    fn resulting_string(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abc", "c"), ("adcb", ""), ("zadb", "db")];

        for (s, expected) in test_cases {
            assert_eq!(S::resulting_string(s.to_string()), expected);
        }
    }
}
