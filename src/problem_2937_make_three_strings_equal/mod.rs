pub mod common_prefix;

pub trait Solution {
    fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("abc", "abb", "ab"), 2), (("dac", "bac", "cac"), -1)];

        for ((s1, s2, s3), expected) in test_cases {
            assert_eq!(
                S::find_minimum_operations(s1.to_string(), s2.to_string(), s3.to_string()),
                expected,
            );
        }
    }
}
