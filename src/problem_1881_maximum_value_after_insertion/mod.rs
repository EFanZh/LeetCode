pub mod iterative;

pub trait Solution {
    fn max_value(n: String, x: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("99", 9), "999"), (("-13", 2), "-123")];

        for ((n, x), expected) in test_cases {
            assert_eq!(S::max_value(n.to_string(), x), expected);
        }
    }
}
