pub mod iterative;

pub trait Solution {
    fn get_hint(secret: String, guess: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("1807", "7810"), "1A3B"),
            (("1123", "0111"), "1A1B"),
            (("1", "0"), "0A0B"),
            (("1", "1"), "1A0B"),
            (("1122", "1222"), "3A0B"),
        ];

        for ((secret, guess), expected) in test_cases.iter().copied() {
            assert_eq!(S::get_hint(secret.to_string(), guess.to_string()), expected);
        }
    }
}
