pub mod iterative;

pub trait Solution {
    fn decode_message(key: String, message: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                ("the quick brown fox jumps over the lazy dog", "vkbs bs t suepuv"),
                "this is a secret",
            ),
            (
                ("eljuxhpwnyrdgtqkviszcfmabo", "zwx hnfx lqantp mnoeius ycgk vcnjrdb"),
                "the five boxing wizards jump quickly",
            ),
        ];

        for ((key, message), expected) in test_cases {
            assert_eq!(S::decode_message(key.to_string(), message.to_string()), expected);
        }
    }
}
