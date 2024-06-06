pub mod dynamic_programming;

pub trait Solution {
    fn count_texts(pressed_keys: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("22233", 8),
            ("222222222222222222222222222222222222", 82_876_089),
            ("444479999555588866", 3136),
        ];

        for (pressed_keys, expected) in test_cases {
            assert_eq!(S::count_texts(pressed_keys.to_string()), expected);
        }
    }
}
