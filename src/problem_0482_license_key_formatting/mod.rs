pub mod iterative;

pub trait Solution {
    fn license_key_formatting(s: String, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("5F3Z-2e-9-w", 4), "5F3Z-2E9W"),
            (("2-5g-3-J", 2), "2-5G-3J"),
            (("---", 3), ""),
        ];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::license_key_formatting(s.to_string(), k), expected);
        }
    }
}
