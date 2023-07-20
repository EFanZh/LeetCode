pub mod greedy;

pub trait Solution {
    fn maximum_binary_string(binary: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("000110", "111011"), ("01", "01"), ("1", "1")];

        for (binary, expected) in test_cases {
            assert_eq!(S::maximum_binary_string(binary.to_string()), expected);
        }
    }
}
